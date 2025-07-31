use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutResponse {
    pub status: String,
    pub data: Payout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub id: String,
    pub account_id: String,
    pub balance_transaction_id: String,
    pub currency: NameValue,
    pub status: NameValue,
    #[serde(rename = "type")]
    pub payout_type: NameValue,
    pub client_reference_id: String,
    pub bank_id: String,
    pub account_bank_account_id: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub amount: i64,
    pub fee: i64,
    pub account: Account,
    pub balance_transaction: BalanceTransaction,
    pub bank: Bank,
    pub account_bank_account: AccountBankAccount,
    pub metadata: PayoutMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameValueInt {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: NameValue,
    pub account_status: NameValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_accounts: Vec<UserAccount>,
    pub business: Business,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub user_id: String,
    pub account_id: String,
    pub role_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: User,
    pub account: String,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub normalized_user_name: String,
    pub email: String,
    pub normalized_email: String,
    pub email_confirmed: bool,
    pub password_hash: String,
    pub security_stamp: String,
    pub concurrency_stamp: String,
    pub phone_number: String,
    pub phone_number_confirmed: bool,
    pub two_factor_enabled: bool,
    pub lockout_end: Option<DateTime<Utc>>,
    pub lockout_enabled: bool,
    pub access_failed_count: i32,
    pub display_name: String,
    pub avatar_url: String,
    pub date_of_birth: NaiveDate,
    pub user_accounts: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub normalized_name: String,
    pub concurrency_stamp: String,
    pub role_type: NameValueInt,
    pub descriptions: Vec<Description>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub language: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Business {
    pub account_id: String,
    pub business_reg_number: String,
    pub business_name: String,
    pub brand: String,
    pub contact: Contact,
    pub address: Address,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub contact_phone: String,
    pub contact_email: String,
    pub support_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTransaction {
    pub id: String,
    pub account_id: String,
    pub currency: NameValue,
    pub amount: i64,
    pub nett_amount: i64,
    pub fee: i64,
    pub description: String,
    pub transaction_type: NameValue,
    pub available_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub account: Account,
    pub linked_entity_id: String,
    pub linked_entity_type: NameValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub id: String,
    pub code: String,
    pub name: String,
    pub country_code: String,
    pub currency: NameValue,
    pub duitku_bank_code: String,
    pub logo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBankAccount {
    pub id: String,
    pub account_id: String,
    pub bank_id: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub status: NameValue,
    pub verification_notes: String,
    pub verified_by: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub account: Account,
    pub bank: Bank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutMetadata {
    pub external_transaction_id: String,
    pub external_reference: String,
    pub external_status: String,
    pub external_error_message: String,
    pub provider_name: String,
    pub raw_response: String,
    pub external_processed_at: DateTime<Utc>,
    pub additional_data: HashMap<String, String>,
}