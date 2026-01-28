use sqlx::SqlitePool;

use wallet_app::{AppError, AppResult, TopupRequestValidated};
use wallet_domain::{AmountMinor, Currency, IdempotencyKey, UserId};

use crate::{db, service};

struct SeedAccount {
    currency: &'static str,
    amount_major: i64,
}

struct SeedUser {
    user_id: &'static str,
    display_name: &'static str,
    accounts: &'static [SeedAccount],
}

struct SeedTransfer {
    from_user_id: &'static str,
    to_user_id: &'static str,
    currency: &'static str,
    amount_major: i64,
}

const SEED_USERS: &[SeedUser] = &[
    SeedUser {
        user_id: "u01",
        display_name: "Amina",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 10_000,
            },
            SeedAccount {
                currency: "UZS",
                amount_major: 150_000,
            },
        ],
    },
    SeedUser {
        user_id: "u02",
        display_name: "Bekzod",
        accounts: &[
            SeedAccount {
                currency: "UZS",
                amount_major: 20_000,
            },
            SeedAccount {
                currency: "USD",
                amount_major: 250,
            },
        ],
    },
    SeedUser {
        user_id: "u03",
        display_name: "Dilshod",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 22_000,
            },
            SeedAccount {
                currency: "EUR",
                amount_major: 180,
            },
        ],
    },
    SeedUser {
        user_id: "u04",
        display_name: "Malika",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 15_000,
            },
            SeedAccount {
                currency: "UZS",
                amount_major: 12_000,
            },
        ],
    },
    SeedUser {
        user_id: "u05",
        display_name: "Sardor",
        accounts: &[
            SeedAccount {
                currency: "UZS",
                amount_major: 30_000,
            },
            SeedAccount {
                currency: "USD",
                amount_major: 140,
            },
        ],
    },
    SeedUser {
        user_id: "u06",
        display_name: "Nargiza",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 18_000,
            },
            SeedAccount {
                currency: "USD",
                amount_major: 90,
            },
        ],
    },
    SeedUser {
        user_id: "u07",
        display_name: "Aziza",
        accounts: &[
            SeedAccount {
                currency: "UZS",
                amount_major: 25_000,
            },
            SeedAccount {
                currency: "RUB",
                amount_major: 14_000,
            },
        ],
    },
    SeedUser {
        user_id: "u08",
        display_name: "Timur",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 12_000,
            },
            SeedAccount {
                currency: "EUR",
                amount_major: 75,
            },
        ],
    },
    SeedUser {
        user_id: "u09",
        display_name: "Rustam",
        accounts: &[
            SeedAccount {
                currency: "UZS",
                amount_major: 40_000,
            },
            SeedAccount {
                currency: "EUR",
                amount_major: 210,
            },
        ],
    },
    SeedUser {
        user_id: "u10",
        display_name: "Sevara",
        accounts: &[
            SeedAccount {
                currency: "RUB",
                amount_major: 50_000,
            },
            SeedAccount {
                currency: "UZS",
                amount_major: 15_000,
            },
            SeedAccount {
                currency: "USD",
                amount_major: 320,
            },
        ],
    },
];

const SEED_TRANSFERS: &[SeedTransfer] = &[
    SeedTransfer {
        from_user_id: "u01",
        to_user_id: "u04",
        currency: "RUB",
        amount_major: 350,
    },
    SeedTransfer {
        from_user_id: "u04",
        to_user_id: "u07",
        currency: "UZS",
        amount_major: 5_000,
    },
    SeedTransfer {
        from_user_id: "u02",
        to_user_id: "u05",
        currency: "USD",
        amount_major: 40,
    },
    SeedTransfer {
        from_user_id: "u03",
        to_user_id: "u09",
        currency: "EUR",
        amount_major: 25,
    },
];

pub fn dev_seed_enabled() -> bool {
    matches!(std::env::var("WALLET_DEV_SEED"), Ok(v) if v == "1")
}

pub async fn seed_demo_data(pool: &SqlitePool) -> AppResult<()> {
    for user in SEED_USERS {
        let user_id = UserId::parse(user.user_id).map_err(AppError::from)?;
        let account_id = db::ensure_user_account(pool, user_id.as_str()).await?;
        db::set_account_label(pool, account_id, user.display_name).await?;

        for account in user.accounts {
            let currency = Currency::parse(account.currency).map_err(AppError::from)?;
            service::admin_open_currency_account(pool, &user_id, &currency).await?;
            let amount_minor =
                amount_minor_from_major(pool, &currency, account.amount_major).await?;
            seed_topup(pool, &user_id, &currency, amount_minor).await?;
        }
    }

    for transfer in SEED_TRANSFERS {
        let from_user = UserId::parse(transfer.from_user_id).map_err(AppError::from)?;
        let to_user = UserId::parse(transfer.to_user_id).map_err(AppError::from)?;
        let currency = Currency::parse(transfer.currency).map_err(AppError::from)?;
        let amount_minor = amount_minor_from_major(pool, &currency, transfer.amount_major).await?;
        seed_transfer(pool, &from_user, &to_user, &currency, amount_minor).await?;
    }

    Ok(())
}

pub async fn list_dev_users(pool: &SqlitePool) -> AppResult<Vec<wallet_app::DevUserItem>> {
    let rows = sqlx::query_as::<_, (String, String)>(
        "SELECT owner_id, label FROM accounts WHERE owner_type = 'user' ORDER BY owner_id ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(user_id, label)| wallet_app::DevUserItem {
            display_name: if label.trim().is_empty() {
                user_id.clone()
            } else {
                label
            },
            user_id,
        })
        .collect())
}

pub async fn list_dev_user_accounts(
    pool: &SqlitePool,
    user_id: &UserId,
) -> AppResult<Vec<wallet_app::DevAccountItem>> {
    let account_id = db::get_user_account_id_or_404(pool, user_id.as_str()).await?;
    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, Option<i64>)>(
        "SELECT ca.id, ca.currency, bp.available_minor, bp.hold_minor
         FROM currency_accounts ca
         LEFT JOIN balance_projection bp
           ON bp.account_id = ca.root_account_id AND bp.currency = ca.currency
         WHERE ca.root_account_id = ?1
         ORDER BY ca.currency ASC",
    )
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(id, currency, available_minor, hold_minor)| wallet_app::DevAccountItem {
                account_id: format!("acc_{}", id),
                currency,
                available_minor: available_minor.unwrap_or(0),
                hold_minor: hold_minor.unwrap_or(0),
            },
        )
        .collect())
}

async fn seed_topup(
    pool: &SqlitePool,
    user_id: &UserId,
    currency: &Currency,
    amount_minor: i64,
) -> AppResult<()> {
    let idem = IdempotencyKey::parse(&format!(
        "seed:{}:{}:topup",
        user_id.as_str(),
        currency.as_str()
    ))
    .map_err(AppError::from)?;
    let amount = AmountMinor::try_from(amount_minor).map_err(AppError::from)?;
    let req = TopupRequestValidated {
        currency: currency.clone(),
        amount_minor: amount,
    };

    let mut tx = pool.begin().await?;
    service::topup_posted(&mut tx, user_id, &idem, req).await?;
    tx.commit().await?;
    Ok(())
}

async fn seed_transfer(
    pool: &SqlitePool,
    from_user_id: &UserId,
    to_user_id: &UserId,
    currency: &Currency,
    amount_minor: i64,
) -> AppResult<()> {
    let idem = IdempotencyKey::parse(&format!(
        "seed:transfer:{}:{}:{}",
        from_user_id.as_str(),
        to_user_id.as_str(),
        currency.as_str()
    ))
    .map_err(AppError::from)?;
    let amount = AmountMinor::try_from(amount_minor).map_err(AppError::from)?;
    let req = wallet_app::TransferRequestValidated {
        to_user_id: to_user_id.clone(),
        currency: currency.clone(),
        amount_minor: amount,
    };

    let mut tx = pool.begin().await?;
    service::transfer_posted(&mut tx, from_user_id, &idem, req).await?;
    tx.commit().await?;
    Ok(())
}

async fn amount_minor_from_major(
    pool: &SqlitePool,
    currency: &Currency,
    amount_major: i64,
) -> AppResult<i64> {
    let minor_units = currency_minor_units(pool, currency).await?;
    let factor = 10_i64
        .checked_pow(minor_units as u32)
        .ok_or(AppError::Internal("minor units overflow"))?;
    amount_major
        .checked_mul(factor)
        .ok_or(AppError::Internal("amount overflow"))
}

async fn currency_minor_units(pool: &SqlitePool, currency: &Currency) -> AppResult<i64> {
    let minor_units =
        sqlx::query_scalar::<_, i64>("SELECT minor_units FROM currencies WHERE code = ?1 LIMIT 1")
            .bind(currency.as_str())
            .fetch_optional(pool)
            .await?
            .ok_or(AppError::UnsupportedCurrency)?;
    Ok(minor_units)
}
