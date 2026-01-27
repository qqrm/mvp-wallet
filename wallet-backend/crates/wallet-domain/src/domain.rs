//! Domain types and validation.

// ========================
// Core domain (pure)
// ========================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainError {
    BadRequest(&'static str),
    Internal(&'static str),
}

pub type DomainResult<T> = Result<T, DomainError>;

// ------------------------ internal domain enums ------------------------

/// Direction of a ledger entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryDirection {
    Debit,
    Credit,
}

impl EntryDirection {
    pub const fn as_str(self) -> &'static str {
        match self {
            EntryDirection::Debit => "debit",
            EntryDirection::Credit => "credit",
        }
    }

    pub fn from_db_str(s: &str) -> DomainResult<Self> {
        match s {
            "debit" => Ok(EntryDirection::Debit),
            "credit" => Ok(EntryDirection::Credit),
            _ => Err(DomainError::Internal("unknown entry direction")),
        }
    }

    pub const fn apply_sign(self, amount_minor_abs: i64) -> i64 {
        match self {
            EntryDirection::Debit => -amount_minor_abs,
            EntryDirection::Credit => amount_minor_abs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxType {
    Topup,
    Transfer,
    Payment,
    Refund,
}

impl TxType {
    pub const fn as_str(self) -> &'static str {
        match self {
            TxType::Topup => "topup",
            TxType::Transfer => "transfer",
            TxType::Payment => "payment",
            TxType::Refund => "refund",
        }
    }

    /// Lossy parsing is acceptable only for historical DB rows / unknown future values.
    pub fn from_db_str_lossy(s: &str) -> Option<Self> {
        match s {
            "topup" => Some(TxType::Topup),
            "transfer" => Some(TxType::Transfer),
            "payment" => Some(TxType::Payment),
            "refund" => Some(TxType::Refund),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    Authorized,
    Posted,
    Reversed,
    Refunded,
}

impl TxState {
    pub const fn as_str(self) -> &'static str {
        match self {
            TxState::Authorized => "authorized",
            TxState::Posted => "posted",
            TxState::Reversed => "reversed",
            TxState::Refunded => "refunded",
        }
    }

    pub fn from_db_str_lossy(s: &str) -> Option<Self> {
        match s {
            "authorized" => Some(TxState::Authorized),
            "posted" => Some(TxState::Posted),
            "reversed" => Some(TxState::Reversed),
            "refunded" => Some(TxState::Refunded),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Closed,
}

impl AccountStatus {
    pub const fn as_db_str(self) -> &'static str {
        match self {
            AccountStatus::Active => "active",
            AccountStatus::Closed => "closed",
        }
    }

    pub const fn as_api_str(self) -> &'static str {
        match self {
            AccountStatus::Active => "ACTIVE",
            AccountStatus::Closed => "CLOSED",
        }
    }

    pub fn from_db_str_lossy(s: &str) -> Option<Self> {
        match s {
            "active" => Some(AccountStatus::Active),
            "closed" => Some(AccountStatus::Closed),
            _ => None,
        }
    }
}

pub const SYSTEM_ACCOUNT_SINK: &str = "SYSTEM_SINK";
pub const SYSTEM_ACCOUNT_SPEND: &str = "SYSTEM_SPEND";

pub fn system_account_owner_id(kind: &str, currency: &Currency) -> String {
    format!("{kind}:{}", currency.as_str())
}

// ------------------------ internal domain newtypes ------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserId(String);

impl UserId {
    pub fn parse(s: &str) -> DomainResult<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(DomainError::BadRequest("user_id is empty"));
        }
        if s.len() > 64 {
            return Err(DomainError::BadRequest("user_id too long"));
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn parse(s: &str) -> DomainResult<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(DomainError::BadRequest("Empty Idempotency-Key header"));
        }
        // Keep it reasonably bounded to avoid abuse; still plenty for UUID-like keys.
        if s.len() > 256 {
            return Err(DomainError::BadRequest("Idempotency-Key too long"));
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Currency(String);

impl Currency {
    pub fn parse(s: &str) -> DomainResult<Self> {
        let s = s.trim();
        if s.len() < 3 || s.len() > 8 {
            return Err(DomainError::BadRequest("invalid currency"));
        }
        // Canonical form: uppercase
        Ok(Self(s.to_ascii_uppercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountId(i64);

impl AccountId {
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    pub const fn get(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmountMinor(u64);

impl AmountMinor {
    pub const MAX: u64 = 1_000_000_000_000;

    pub fn new(value: u64) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::BadRequest("amount_minor must be > 0"));
        }
        if value > Self::MAX {
            return Err(DomainError::BadRequest("amount_minor too large"));
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub const fn as_i64(self) -> i64 {
        self.0 as i64
    }
}

impl TryFrom<i64> for AmountMinor {
    type Error = DomainError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= 0 {
            return Err(DomainError::BadRequest("amount_minor must be > 0"));
        }
        AmountMinor::new(value as u64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TxId(uuid::Uuid);

impl TxId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse(s: &str) -> DomainResult<Self> {
        let u = uuid::Uuid::parse_str(s).map_err(|_| DomainError::BadRequest("invalid tx_id"))?;
        Ok(Self(u))
    }

    pub fn to_string_hyphenated(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for TxId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TxId::parse(s)
    }
}
