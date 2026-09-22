use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub order_number: String,
    pub vehicle_id: String,
    pub amount: i64,
    pub payment_type: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddTransaction {
    pub order_number: String,
    pub vehicle_id: String,
    pub amount: i64,
    pub payment_type: String,
    pub created_by: String,
}
