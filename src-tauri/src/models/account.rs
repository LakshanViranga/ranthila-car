use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddAccountSummery {
    pub date: String,
    pub income_cash: i64,
    pub income_credit: i64,
    pub income_bank_transfer: i64,
    pub expenses_cash: i64,
    pub expenses_credit: i64,
    pub expenses_bank: i64,
    pub bank_deposit: i64,
    pub hand_on_cash: i64,
    pub created_by: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountSummery {
    pub date: String,
    pub income_cash: i64,
    pub income_credit: i64,
    pub income_bank_transfer: i64,
    pub expenses_cash: i64,
    pub expenses_credit: i64,
    pub expenses_bank: i64,
    pub bank_deposit: i64,
    pub hand_on_cash: i64,
    pub created_by: String,
    pub created_at: String,
}
