# Wallet MVP (backend + web)

This repository contains:
- `wallet-backend/`: Rust workspace (domain/app/infra/api)
- `wallet-web/`: Vite web UI

## Perf testing the processing core (throughput + latency)

### What this measures

The wallet "core" is currently DB-centric (idempotency, double-entry persistence, projection updates). A realistic TPS number therefore includes SQL transaction begin/commit and the required reads/writes. The harness below calls `wallet-infra/src/service.rs` directly (no HTTP/JSON), so it removes network/serialization noise while keeping production semantics.

If you need a **CPU-only** comparable that minimizes environmental variance ("how many instructions do we execute to build ledger deltas"), use the microbench section further down.

### Run locally

From repo root:

```bash
# Mixed topup/transfer/hold roundtrips, 4 workers, 20s measurement, 5s warmup
just backend-perf --scenario mixed --workers 4 --duration-secs 20 --warmup-secs 5
```

Direct cargo invocation (same thing):

```bash
cargo run --manifest-path wallet-backend/Cargo.toml -p wallet-perf -- \
  --scenario mixed --workers 4 --duration-secs 20 --warmup-secs 5
```

Useful flags:
- `--fresh`: remove DB file before run
- `--db <path>`: DB path (default `wallet-backend/target/wallet-perf.db`)
- `--users <N>`: how many user accounts to seed
- `--prefund-minor <N>`: prefund each user (needed for transfers/holds)
- `--json` and/or `--out report.json`: machine-readable report
- `--no-invariants`: skip post-run invariants checks

### Run in an isolated 1 CPU / 1GiB envelope (Docker)

Build and run from repo root:

```bash
docker build -f wallet-backend/Dockerfile.perf -t wallet-perf wallet-backend

# Strict resource envelope: 1 CPU, 1GiB RAM.
# Use workers=1 for a "single core" view.
docker run --rm --cpus=1 --memory=1g wallet-perf \
  --scenario mixed --workers 1 --duration-secs 30 --warmup-secs 5 --fresh
```

Notes:
- SQLite is **single-writer**. With `--workers > 1`, higher contention is expected; that's still useful as a stress signal, but it's not how Postgres behaves.
- For "single-core" numbers, use `--cpus=1` (or `--cpuset-cpus=0`) and `--workers=1`.

### Run in Kubernetes (Job template)

See `wallet-backend/k8s/wallet-perf-job.yaml` (CPU=1, memory=1Gi). You can adjust args and image name.

### CPU-only microbench (DB-agnostic)

This benchmarks the pure domain-core bookkeeping helpers (`wallet-domain/src/core.rs`) and is useful for comparing code changes without DB noise:

```bash
cargo bench --manifest-path wallet-backend/Cargo.toml -p wallet-perf --bench domain_core
```

## Interpreting results

The harness prints:
- `ops/sec` over the measurement window
- latency `p50/p95/p99` (microseconds) for an operation including commit
- error breakdown (conflicts, locked/busy, etc.)

A "universal" metric across different machines is not a single number. In practice you standardize:
1) a workload (scenario + amounts + users)
2) a resource envelope (CPU/memory limits)
3) an SLO (e.g., p99 < X ms)

Then you track regressions and capacity curves under that fixed definition.
