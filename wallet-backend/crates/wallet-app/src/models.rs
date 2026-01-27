//! HTTP-facing request/response models and validation.

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::AppError;

use wallet_domain::*;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CurrencyItem {
    pub code: String,
    pub minor_units: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListCurrenciesResponse {
    pub items: Vec<CurrencyItem>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TopupRequest {
    pub currency: String,
    pub amount_minor: i64,
}

#[derive(Debug, Clone)]
pub struct TopupRequestValidated {
    pub currency: Currency,
    pub amount_minor: AmountMinor,
}

impl TryFrom<TopupRequest> for TopupRequestValidated {
    type Error = AppError;

    fn try_from(value: TopupRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
            amount_minor: AmountMinor::try_from(value.amount_minor).map_err(AppError::from)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TransferRequest {
    pub to_user_id: String,
    pub currency: String,
    pub amount_minor: i64,
}

#[derive(Debug, Clone)]
pub struct TransferRequestValidated {
    pub to_user_id: UserId,
    pub currency: Currency,
    pub amount_minor: AmountMinor,
}

impl TryFrom<TransferRequest> for TransferRequestValidated {
    type Error = AppError;

    fn try_from(value: TransferRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            to_user_id: UserId::parse(&value.to_user_id).map_err(AppError::from)?,
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
            amount_minor: AmountMinor::try_from(value.amount_minor).map_err(AppError::from)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BalanceItem {
    pub currency: String,
    pub available_minor: i64,
    pub hold_minor: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListBalancesResponse {
    pub balances: Vec<BalanceItem>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TxItem {
    pub tx_id: String,
    pub tx_type: String,
    pub state: String,
    pub currency: String,
    pub amount_minor: i64,
    pub created_at: String,
    pub posted_at: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TxReceiptEntryItem {
    /// "user" | "system"
    pub party_type: String,
    /// user_id or system owner_id
    pub party_id: String,
    /// optional label from accounts.label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_label: Option<String>,
    pub currency: String,
    /// "debit" | "credit"
    pub direction: String,
    /// absolute (positive)
    pub amount_minor: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TxReceiptResponse {
    pub tx_id: String,
    pub tx_type: String,
    pub state: String,
    pub currency: String,
    pub amount_minor: i64,
    pub created_at: String,
    pub posted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_minor: Option<i64>,
    pub entries: Vec<TxReceiptEntryItem>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListTxsQuery {
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListTxsResponse {
    pub txs: Vec<TxItem>,
}

// ------------------------ Admin (MVP) ------------------------

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminCreateUserRequest {
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminCreateUserResponse {
    pub user_id: String,
    pub created: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminOpenCurrencyAccountRequest {
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminOpenCurrencyAccountResponse {
    pub user_id: String,
    pub currency: String,
    pub opened: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminTopupRequest {
    pub user_id: String,
    pub currency: String,
    pub amount_minor: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminHoldRequest {
    pub user_id: String,
    pub currency: String,
    pub amount_minor: i64,
}

// ------------------------ Admin Account Lifecycle (new) ------------------------

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminCreateAccountRequest {
    pub owner_user_id: String,
    pub currency: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminCreateAccountResponse {
    pub account_id: String,
    pub status: String,
    pub currency: String,
    pub owner_user_id: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminCloseAccountResponse {
    pub account_id: String,
    pub status: String,
    pub closed_at: String,
}

#[derive(Debug, Clone)]
pub struct AdminCreateAccountRequestValidated {
    pub owner_user_id: UserId,
    pub currency: Currency,
    pub label: String,
}

impl TryFrom<AdminCreateAccountRequest> for AdminCreateAccountRequestValidated {
    type Error = AppError;

    fn try_from(value: AdminCreateAccountRequest) -> Result<Self, Self::Error> {
        let label = value.label.trim().to_string();
        if label.len() > 128 {
            return Err(AppError::BadRequest("label too long"));
        }

        Ok(Self {
            owner_user_id: UserId::parse(&value.owner_user_id).map_err(AppError::from)?,
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
            label,
        })
    }
}

// ------------------------ typed requests ------------------------

#[derive(Debug, Clone)]
pub struct AdminOpenCurrencyValidated {
    pub currency: Currency,
}

impl TryFrom<AdminOpenCurrencyAccountRequest> for AdminOpenCurrencyValidated {
    type Error = AppError;

    fn try_from(value: AdminOpenCurrencyAccountRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdminTopupRequestValidated {
    pub user_id: UserId,
    pub currency: Currency,
    pub amount_minor: AmountMinor,
}

impl TryFrom<AdminTopupRequest> for AdminTopupRequestValidated {
    type Error = AppError;

    fn try_from(value: AdminTopupRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::parse(&value.user_id).map_err(AppError::from)?,
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
            amount_minor: AmountMinor::try_from(value.amount_minor).map_err(AppError::from)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HoldRequestValidated {
    pub currency: Currency,
    pub amount_minor: AmountMinor,
}

impl TryFrom<AdminHoldRequest> for HoldRequestValidated {
    type Error = AppError;

    fn try_from(value: AdminHoldRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            currency: Currency::parse(&value.currency).map_err(AppError::from)?,
            amount_minor: AmountMinor::try_from(value.amount_minor).map_err(AppError::from)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostOpResponse {
    pub tx_id: String,
    pub state: String,
    pub currency: String,
    pub amount_minor: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RefundResponse {
    pub original_tx_id: String,
    pub refund_tx_id: String,
    pub state: String,
    pub currency: String,
    pub amount_minor: i64,
}

// ------------------------ v3.3 (SoT) HTTP models ------------------------

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProfileResponse {
    pub phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AccountItemV33 {
    pub account_id: String, // "acc_<id>"
    pub currency: String,
    pub status: String, // 'active' | 'closed'
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListAccountsResponseV33 {
    pub accounts: Vec<AccountItemV33>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AccountBalanceResponseV33 {
    pub account_id: String,
    pub currency: String,
    pub available_minor: i64,
    pub hold_minor: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AccountDetailsResponseV33 {
    pub account_id: String, // "acc_<id>"
    pub currency: String,
    pub status: String,
    pub label: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    pub balance: AccountBalanceResponseV33,
    // Admin-only field (best effort) if the endpoint is accessed with admin auth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct ListAccountTransactionsQueryV33 {
    /// Return transactions strictly before this timestamp (RFC3339). If absent, returns newest.
    pub before: Option<String>,
    /// Number of items to return. Default is 100.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListAccountTransactionsResponseV33 {
    pub account_id: String,
    pub txs: Vec<TxItem>,
}
