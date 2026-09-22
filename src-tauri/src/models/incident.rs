use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incident {
    pub id: i64,
    pub order_number: String,
    pub customer_id: String,
    pub incident_type: String,
    pub severity: String,
    pub description: String,
    pub estimation_cost: i64,
    pub incident_date: String,
    pub status: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddIncident {
    pub order_number: String,
    pub customer_id: String,
    pub incident_type: String,
    pub severity: String,
    pub description: String,
    pub estimation_cost: i64,
    pub incident_date: String,
    pub status: String,
    pub created_by: String,
}
