//! wallet-perf: component-level throughput/latency harness for the wallet "processing core".
//!
//! Design goals:
//! - Calls wallet-infra service functions directly (no HTTP/JSON).
//! - Uses real SQL transactions (begin/commit) because this is what production does.
//! - Provides stable, resource-envelope based measurement (run under docker/k8s CPU/memory limits).
//! - Emits latency p50/p95/p99 + ops/sec + error breakdown.
//!
//! Important limitation:
//! In the current architecture, the processing core is DB-centric (idempotency, projection, invariants).
//! "Code-only" TPS without *any* storage is not representative. For CPU-only comparables, see
//! `cargo bench -p wallet-perf` (domain-core microbench).

use std::{
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::{Parser, ValueEnum};
use hdrhistogram::Histogram;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand::RngCore;
use serde::Serialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use tokio::{sync::Barrier, task::JoinSet};

use wallet_app::{AppError, HoldRequestValidated, TopupRequestValidated, TransferRequestValidated};
use wallet_domain::{AmountMinor, Currency, IdempotencyKey, TxId, UserId};
use wallet_infra::{db, service};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Scenario {
    /// Random topups into random users.
    Topup,
    /// Random transfers between users (requires prefunding users).
    Transfer,
    /// Payment hold authorize, immediately followed by capture or cancel.
    HoldRoundtrip,
    /// Mixed traffic: topup/transfer/hold-roundtrip.
    Mixed,
}

#[derive(Debug, Parser)]
#[command(name = "wallet-perf", version, about = "Wallet processing-core perf harness (no HTTP)")]
struct Cli {
    /// SQLite DB file path. Default is wallet-backend/target/wallet-perf.db.
    #[arg(long, default_value = "target/wallet-perf.db")]
    db: String,

    /// Deletes the DB file (and WAL/SHM) before running.
    #[arg(long, default_value_t = false)]
    fresh: bool,

    /// Scenario / traffic mix.
    #[arg(long, value_enum, default_value_t = Scenario::Mixed)]
    scenario: Scenario,

    /// Number of concurrent workers.
    #[arg(long, default_value_t = 4)]
    workers: usize,

    /// Size of SQL pool (SQLite still serializes writes, but pool helps with scheduling).
    #[arg(long, default_value_t = 8)]
    pool_max: u32,

    /// Number of users to seed.
    #[arg(long, default_value_t = 200)]
    users: usize,

    /// Currency used for all operations.
    #[arg(long, default_value = "USD")]
    currency: String,

    /// Prefund amount for each user (needed for transfer/holds). Set to 0 to skip prefunding.
    #[arg(long, default_value_t = 1_000_000)]
    prefund_minor: i64,

    /// Duration of warmup phase.
    #[arg(long, default_value_t = 5)]
    warmup_secs: u64,

    /// Duration of measurement phase.
    #[arg(long, default_value_t = 20)]
    duration_secs: u64,

    /// RNG seed for determinism.
    #[arg(long, default_value_t = 1)]
    seed: u64,

    /// Print a machine-readable JSON report to stdout.
    #[arg(long, default_value_t = false)]
    json: bool,

    /// Write JSON report to this path (in addition to stdout text output).
    #[arg(long)]
    out: Option<String>,

    /// Skip post-run invariants checks (faster, but less safe).
    #[arg(long, default_value_t = false)]
    no_invariants: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ErrorBreakdown {
    conflict: u64,
    bad_request: u64,
    not_found: u64,
    db_locked_or_busy: u64,
    other_db: u64,
    other: u64,
}

impl ErrorBreakdown {
    fn bump(&mut self, e: &AppError) {
        match e {
            AppError::Conflict(_) => self.conflict += 1,
            AppError::BadRequest(_) => self.bad_request += 1,
            AppError::NotFound(_) => self.not_found += 1,
            AppError::Db(db_e) => {
                // sqlite busy/locked are surfaced as string messages in sqlx errors.
                let s = db_e.to_string().to_ascii_lowercase();
                if s.contains("database is locked") || s.contains("database is busy") {
                    self.db_locked_or_busy += 1;
                } else {
                    self.other_db += 1;
                }
            }
            _ => self.other += 1,
        }
    }
}

#[derive(Debug)]
struct WorkerStats {
    ok: u64,
    err: u64,
    lat_us: Histogram<u64>,
    errors: ErrorBreakdown,
}

impl WorkerStats {
    fn new() -> Self {
        Self {
            ok: 0,
            err: 0,
            lat_us: Histogram::<u64>::new(3).expect("hist"),
            errors: ErrorBreakdown {
                conflict: 0,
                bad_request: 0,
                not_found: 0,
                db_locked_or_busy: 0,
                other_db: 0,
                other: 0,
            },
        }
    }

    fn record_ok(&mut self, dur: Duration) {
        self.ok += 1;
        let us = dur.as_micros().min(u128::from(u64::MAX)) as u64;
        let _ = self.lat_us.record(us);
    }

    fn record_err(&mut self, dur: Duration, e: &AppError) {
        self.err += 1;
        let us = dur.as_micros().min(u128::from(u64::MAX)) as u64;
        let _ = self.lat_us.record(us);
        self.errors.bump(e);
    }

    fn merge_into(self, agg: &mut Aggregate) {
        agg.ok += self.ok;
        agg.err += self.err;
        agg.errors.conflict += self.errors.conflict;
        agg.errors.bad_request += self.errors.bad_request;
        agg.errors.not_found += self.errors.not_found;
        agg.errors.db_locked_or_busy += self.errors.db_locked_or_busy;
        agg.errors.other_db += self.errors.other_db;
        agg.errors.other += self.errors.other;
        agg.lat_us.add(&self.lat_us).expect("merge");
    }
}

#[derive(Debug, Serialize)]
struct Aggregate {
    ok: u64,
    err: u64,
    lat_us: Histogram<u64>,
    errors: ErrorBreakdown,
}

impl Aggregate {
    fn new() -> Self {
        Self {
            ok: 0,
            err: 0,
            lat_us: Histogram::<u64>::new(3).expect("hist"),
            errors: ErrorBreakdown {
                conflict: 0,
                bad_request: 0,
                not_found: 0,
                db_locked_or_busy: 0,
                other_db: 0,
                other: 0,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct Report {
    scenario: String,
    db: String,
    workers: usize,
    pool_max: u32,
    users: usize,
    currency: String,
    warmup_secs: u64,
    duration_secs: u64,
    seed: u64,

    ok: u64,
    err: u64,
    ops_per_sec: f64,

    p50_us: u64,
    p95_us: u64,
    p99_us: u64,

    errors: ErrorBreakdown,

    notes: Vec<String>,
}

fn delete_db_files(path: &str) {
    let base = Path::new(path);
    let _ = std::fs::remove_file(base);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-shm"));
}

async fn create_pool(db_path: &str, pool_max: u32) -> anyhow::Result<SqlitePool> {
    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(pool_max)
        .connect_with(opts)
        .await
        .with_context(|| format!("connect sqlite at {db_path}"))?;

    // Pragmas that tend to reduce noise (still real DB semantics).
    sqlx::query("PRAGMA temp_store = MEMORY;").execute(&pool).await?;
    sqlx::query("PRAGMA cache_size = -200000;").execute(&pool).await?; // ~200MB cache

    Ok(pool)
}

async fn seed_users_and_currency(pool: &SqlitePool, users: usize, currency: &Currency) -> anyhow::Result<Vec<UserId>> {
    db::migrate(pool).await.context("migrate")?;
    db::ensure_system_accounts(pool).await.context("ensure_system_accounts")?;

    let mut out = Vec::with_capacity(users);
    for i in 0..users {
        let u = UserId::parse(&format!("u{i:06}"))
            .map_err(|e| anyhow::anyhow!("bad user id: {e:?}"))?;
        let _ = service::admin_create_user(pool, &u).await?;
        let _ = service::admin_open_currency_account(pool, &u, currency).await?;
        out.push(u);
    }

    Ok(out)
}

async fn prefund_users(pool: &SqlitePool, users: &[UserId], currency: Currency, prefund_minor: i64) -> anyhow::Result<()> {
    if prefund_minor <= 0 {
        return Ok(());
    }

    for (i, u) in users.iter().enumerate() {
        let key = IdempotencyKey::parse(&format!("prefund-{i}"))
            .map_err(|e| anyhow::anyhow!("bad idem key: {e:?}"))?;
        let req = TopupRequestValidated {
            currency: currency.clone(),
            amount_minor: AmountMinor::try_from(prefund_minor)
                .map_err(|e| anyhow::anyhow!("bad amount: {e:?}"))?,
        };

        let mut tx = pool.begin().await?;
        let _ = service::topup_posted(&mut tx, u, &key, req).await?;
        tx.commit().await?;
    }

    Ok(())
}

async fn op_topup(
    pool: &SqlitePool,
    user: &UserId,
    currency: &Currency,
    amount_minor: i64,
    idem_key: &IdempotencyKey,
) -> Result<(), AppError> {
    let req = TopupRequestValidated {
        currency: currency.clone(),
        amount_minor: AmountMinor::try_from(amount_minor).map_err(AppError::from)?,
    };

    let mut tx = pool.begin().await.map_err(AppError::from)?;
    let _ = service::topup_posted(&mut tx, user, idem_key, req).await?;
    tx.commit().await.map_err(AppError::from)?;
    Ok(())
}

async fn op_transfer(
    pool: &SqlitePool,
    from: &UserId,
    to: &UserId,
    currency: &Currency,
    amount_minor: i64,
    idem_key: &IdempotencyKey,
) -> Result<(), AppError> {
    let req = TransferRequestValidated {
        to_user_id: to.clone(),
        currency: currency.clone(),
        amount_minor: AmountMinor::try_from(amount_minor).map_err(AppError::from)?,
    };

    let mut tx = pool.begin().await.map_err(AppError::from)?;
    let _ = service::transfer_posted(&mut tx, from, idem_key, req).await?;
    tx.commit().await.map_err(AppError::from)?;
    Ok(())
}

async fn op_hold_authorize(
    pool: &SqlitePool,
    user: &UserId,
    req: HoldRequestValidated,
    idem_key: &IdempotencyKey,
) -> Result<TxId, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::from)?;
    let (_resp, tx_id) = service::payment_hold_authorize(&mut tx, user, idem_key, req).await?;
    tx.commit().await.map_err(AppError::from)?;
    Ok(tx_id)
}

async fn op_hold_settle(pool: &SqlitePool, tx_id: &TxId, capture: bool) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(AppError::from)?;
    if capture {
        let _ = service::payment_hold_capture(&mut tx, tx_id).await?;
    } else {
        let _ = service::payment_hold_cancel(&mut tx, tx_id).await?;
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(())
}

async fn check_invariants(pool: &SqlitePool) -> anyhow::Result<()> {
    // 1) Every tx must have zero-sum ledger entries.
    let bad: Option<(String, i64)> = sqlx::query_as(
        "SELECT tx_id, SUM(CASE direction WHEN 'debit' THEN -amount_minor ELSE amount_minor END) AS s\n         FROM ledger_entries\n         GROUP BY tx_id\n         HAVING s != 0\n         LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some((tx_id, s)) = bad {
        anyhow::bail!("invariant failed: tx {tx_id} is not zero-sum (sum={s})");
    }

    // 2) For each currency: sum(available + hold) must remain zero.
    let bad2: Option<(String, i64)> = sqlx::query_as(
        "SELECT currency, SUM(available_minor + hold_minor) AS s\n         FROM balance_projection\n         GROUP BY currency\n         HAVING s != 0\n         LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some((currency, s)) = bad2 {
        anyhow::bail!("invariant failed: currency {currency} total (available+hold) != 0 (sum={s})");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.fresh {
        delete_db_files(&cli.db);
    }

    let currency = Currency::parse(&cli.currency)
        .map_err(|e| anyhow::anyhow!("invalid currency: {e:?}"))?;

    // SQLite is single-writer. Large worker counts will mostly measure lock contention.
    // This is still useful, but interpret numbers carefully.
    let pool = create_pool(&cli.db, cli.pool_max).await?;

    let users = seed_users_and_currency(&pool, cli.users, &currency).await?;

    // Prefund if needed for transfers/holds.
    if matches!(cli.scenario, Scenario::Transfer | Scenario::HoldRoundtrip | Scenario::Mixed) {
        prefund_users(&pool, &users, currency.clone(), cli.prefund_minor).await?;
    }

    let warmup_deadline = Instant::now() + Duration::from_secs(cli.warmup_secs);
    let measure_deadline = warmup_deadline + Duration::from_secs(cli.duration_secs);

    let barrier = Arc::new(Barrier::new(cli.workers));
    let stop = Arc::new(AtomicU64::new(0));

    let mut joins = JoinSet::new();
    for wid in 0..cli.workers {
        let pool = pool.clone();
        let users = users.clone();
        let currency = currency.clone();
        let barrier = barrier.clone();
        let stop = stop.clone();
        let scenario = cli.scenario;
        let seed = cli.seed;

        joins.spawn(async move {
            let mut rng = StdRng::seed_from_u64(seed ^ (wid as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let mut stats = WorkerStats::new();

            barrier.wait().await;

            loop {
                let now = Instant::now();
                if now >= measure_deadline {
                    stop.store(1, Ordering::Relaxed);
                    break;
                }

                // Warmup phase: run same operations but do not count.
                let count = now >= warmup_deadline;

                let start = Instant::now();
                let res: Result<(), AppError> = match scenario {
                    Scenario::Topup => {
                        let idx = rng.gen_range(0..users.len());
                        let user = &users[idx];
                        let amount = rng.gen_range(1..=1_000);
                        let key = IdempotencyKey::parse(&format!("w{wid}-t-{}", rng.next_u64()))
                            .map_err(AppError::from)?;
                        op_topup(&pool, user, &currency, amount, &key).await
                    }
                    Scenario::Transfer => {
                        let mut a = rng.gen_range(0..users.len());
                        let mut b = rng.gen_range(0..users.len());
                        if users.len() > 1 {
                            while b == a {
                                b = rng.gen_range(0..users.len());
                            }
                        }
                        let from = &users[a];
                        let to = &users[b];
                        let amount = rng.gen_range(1..=1_000);
                        let key = IdempotencyKey::parse(&format!("w{wid}-x-{}", rng.next_u64()))
                            .map_err(AppError::from)?;
                        op_transfer(&pool, from, to, &currency, amount, &key).await
                    }
                    Scenario::HoldRoundtrip => {
                        let idx = rng.gen_range(0..users.len());
                        let user = &users[idx];
                        let amount = rng.gen_range(1..=1_000);
                        let key = IdempotencyKey::parse(&format!("w{wid}-h-{}", rng.next_u64()))
                            .map_err(AppError::from)?;
                        let req = HoldRequestValidated {
                            currency: currency.clone(),
                            amount_minor: AmountMinor::try_from(amount).map_err(AppError::from)?,
                        };
                        let tx_id = op_hold_authorize(&pool, user, req, &key).await?;
                        let capture = rng.gen_bool(0.5);
                        op_hold_settle(&pool, &tx_id, capture).await
                    }
                    Scenario::Mixed => {
                        let r: u32 = rng.gen_range(0..100);
                        if r < 30 {
                            // topup
                            let idx = rng.gen_range(0..users.len());
                            let user = &users[idx];
                            let amount = rng.gen_range(1..=1_000);
                            let key = IdempotencyKey::parse(&format!("w{wid}-mt-{}", rng.next_u64()))
                                .map_err(AppError::from)?;
                            op_topup(&pool, user, &currency, amount, &key).await
                        } else if r < 80 {
                            // transfer
                            let mut a = rng.gen_range(0..users.len());
                            let mut b = rng.gen_range(0..users.len());
                            if users.len() > 1 {
                                while b == a {
                                    b = rng.gen_range(0..users.len());
                                }
                            }
                            let from = &users[a];
                            let to = &users[b];
                            let amount = rng.gen_range(1..=1_000);
                            let key = IdempotencyKey::parse(&format!("w{wid}-mx-{}", rng.next_u64()))
                                .map_err(AppError::from)?;
                            op_transfer(&pool, from, to, &currency, amount, &key).await
                        } else {
                            // hold roundtrip
                            let idx = rng.gen_range(0..users.len());
                            let user = &users[idx];
                            let amount = rng.gen_range(1..=1_000);
                            let key = IdempotencyKey::parse(&format!("w{wid}-mh-{}", rng.next_u64()))
                                .map_err(AppError::from)?;
                            let req = HoldRequestValidated {
                                currency: currency.clone(),
                                amount_minor: AmountMinor::try_from(amount).map_err(AppError::from)?,
                            };
                            let tx_id = op_hold_authorize(&pool, user, req, &key).await?;
                            let capture = rng.gen_bool(0.5);
                            op_hold_settle(&pool, &tx_id, capture).await
                        }
                    }
                };

                let dur = start.elapsed();

                if count {
                    match res {
                        Ok(()) => stats.record_ok(dur),
                        Err(e) => stats.record_err(dur, &e),
                    }
                }

                if stop.load(Ordering::Relaxed) == 1 {
                    break;
                }
            }

            Ok::<_, anyhow::Error>(stats)
        });
    }

    let mut agg = Aggregate::new();
    while let Some(res) = joins.join_next().await {
        let stats = res??;
        stats.merge_into(&mut agg);
    }

    let total = agg.ok + agg.err;
    let ops_per_sec = if cli.duration_secs == 0 {
        0.0
    } else {
        (total as f64) / (cli.duration_secs as f64)
    };

    let p50_us = agg.lat_us.value_at_quantile(0.50);
    let p95_us = agg.lat_us.value_at_quantile(0.95);
    let p99_us = agg.lat_us.value_at_quantile(0.99);

    let mut notes = Vec::new();
    if cli.workers > 1 {
        notes.push("SQLite is single-writer; high worker counts primarily measure write-lock contention".to_string());
    }
    notes.push("For CPU-only comparables, run: cargo bench -p wallet-perf".to_string());

    if !cli.no_invariants {
        check_invariants(&pool).await.context("post-run invariants")?;
    }

    let report = Report {
        scenario: format!("{:?}", cli.scenario).to_lowercase(),
        db: cli.db.clone(),
        workers: cli.workers,
        pool_max: cli.pool_max,
        users: cli.users,
        currency: cli.currency.clone(),
        warmup_secs: cli.warmup_secs,
        duration_secs: cli.duration_secs,
        seed: cli.seed,

        ok: agg.ok,
        err: agg.err,
        ops_per_sec,

        p50_us,
        p95_us,
        p99_us,

        errors: agg.errors.clone(),
        notes,
    };

    if cli.json {
        println!("{}", serde_json::to_string(&report)?);
    } else {
        println!("scenario     : {}", report.scenario);
        println!("db           : {}", report.db);
        println!("workers      : {}", report.workers);
        println!("pool_max     : {}", report.pool_max);
        println!("users        : {}", report.users);
        println!("currency     : {}", report.currency);
        println!("warmup_secs  : {}", report.warmup_secs);
        println!("duration_secs: {}", report.duration_secs);
        println!("seed         : {}", report.seed);
        println!();
        println!("ok           : {}", report.ok);
        println!("err          : {}", report.err);
        println!("ops/sec      : {:.2}", report.ops_per_sec);
        println!("latency_us   : p50={} p95={} p99={}", report.p50_us, report.p95_us, report.p99_us);
        println!();
        println!("errors:");
        println!("  conflict          : {}", report.errors.conflict);
        println!("  bad_request       : {}", report.errors.bad_request);
        println!("  not_found         : {}", report.errors.not_found);
        println!("  db_locked_or_busy : {}", report.errors.db_locked_or_busy);
        println!("  other_db          : {}", report.errors.other_db);
        println!("  other             : {}", report.errors.other);
        println!();
        for n in &report.notes {
            println!("note: {n}");
        }
    }

    if let Some(out) = &cli.out {
        std::fs::write(out, serde_json::to_vec_pretty(&report)?).with_context(|| format!("write report to {out}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_merge_is_monotonic() {
        let mut a = Histogram::<u64>::new(3).unwrap();
        let mut b = Histogram::<u64>::new(3).unwrap();
        for i in 1..=1000u64 {
            a.record(i).unwrap();
            b.record(i * 2).unwrap();
        }
        a.add(&b).unwrap();
        assert!(a.value_at_quantile(0.99) >= a.value_at_quantile(0.95));
        assert!(a.len() > 0);
    }

    #[test]
    fn error_breakdown_classifies_conflict() {
        let mut e = ErrorBreakdown {
            conflict: 0,
            bad_request: 0,
            not_found: 0,
            db_locked_or_busy: 0,
            other_db: 0,
            other: 0,
        };
        e.bump(&AppError::Conflict("x"));
        assert_eq!(e.conflict, 1);
    }
}
