use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMaintenanceRequest {
    pub vehicle_id: String,
    pub service_type: String,
    pub description: String,
    pub cost: i64,
    pub service_provider: String,
    pub service_date: String,
    pub service_mileage: Option<i64>,
    pub next_service_by: Option<String>,
    pub next_service_date: Option<String>,
    pub next_service_mileage: Option<i64>,
    pub payment_status: String,
    pub payment_type: String,
    pub created_by: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaintenanceRecord {
    pub id: i64,
    pub vehicle_id: String,
    pub vehicle_register_number: String,
    pub service_type: String,
    pub description: String,
    pub cost: i64,
    pub service_provider: String,
    pub service_date: String,
    pub service_mileage: Option<i64>,
    pub next_service_by: Option<String>,
    pub next_service_date: Option<String>,
    pub next_service_mileage: Option<i64>,
    pub payment_status: String,
    pub payment_type: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMaintenanceRequest {
    pub id: i64,
    pub service_type: String,
    pub description: String,
    pub service_provider: String,
    pub service_date: String,
    pub service_mileage: Option<i64>,
    pub next_service_by: Option<String>,
    pub next_service_date: Option<String>,
    pub next_service_mileage: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePaymentStatus {
    pub id: i64,
    pub payment_status: String,
}
