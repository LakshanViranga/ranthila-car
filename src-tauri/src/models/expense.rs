use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddExpensesRecord {
    pub category: String,
    pub description: String,
    pub amount: i64,
    pub date: String,
    pub payment_type: String,
    pub payment_status: String,
    pub created_by: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesRecord {
    pub id: i64,
    pub category: String,
    pub description: String,
    pub amount: i64,
    pub date: String,
    pub payment_type: String,
    pub payment_status: String,
    pub created_by: String,
    pub created_at: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateExpensesRecord {
    pub id: i64,
    pub category: String,
    pub description: String,
    pub amount: i64,
    pub date: String,
    pub payment_type: String,
    pub payment_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePaymentStatus {
    pub id: i64,
    pub payment_status: String,
}
