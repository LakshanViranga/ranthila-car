use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBankRecordRequest {
    pub bank_name: String,
    pub amount: Option<i32>,
    pub deposit_date: Option<String>, // Format: YYYY-MM-DD
    pub note: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankRecord {
    pub id: i32,
    pub bank_name: String,
    pub amount: Option<i32>,
    pub deposit_date: Option<String>,
    pub note: Option<String>,
    pub created_at: String,
    pub created_by: Option<String>,
}
