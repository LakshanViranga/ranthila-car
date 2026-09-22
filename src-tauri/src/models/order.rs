use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i32,
    pub order_number: String,
    pub customer_id: String,
    pub customer_name: String,
    pub vehicle_id: String,
    pub starting_mileage: i64,
    pub end_mileage: Option<i32>,
    pub release_time: String,
    pub handover_time: Option<String>,
    pub guarantee_type: Option<String>,
    pub guarantee_property: Option<String>,
    pub customer_image: Option<Vec<u8>>,
    pub contact_no: String,
    pub total_distance: Option<i64>,
    pub total_amount: Option<i32>,
    pub advanced_payment: Option<i64>,
    pub payment_type: Option<String>,
    pub payment_status: Option<String>,
    pub paid_amount: i64,
    pub order_status: Option<String>,
    pub notes: Option<String>,
    pub bank_account_name: Option<String>,
    pub bank_transfer_amount: Option<i64>,
    pub cash_amount: Option<i64>,
    pub discount: Option<i64>,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderRequest {
    pub order_number: String,
    pub customer_id: String,
    pub vehicle_id: String,
    pub starting_mileage: Option<i64>,
    pub release_time: String,
    pub handover_time: Option<String>,
    pub guarantee_type: Option<String>,
    pub guarantee_property: Option<String>,
    pub customer_image: Vec<u8>,
    pub contact_no: String,
    pub total_amount: Option<i32>,
    pub advanced_payment: Option<i64>,
    pub payment_type: String,
    pub payment_status: Option<String>,
    pub paid_amount: i64,
    pub order_status: Option<String>,
    pub notes: Option<String>,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateOrderRequest {
    pub order_number: String,
    pub customer_id: String,
    pub vehicle_id: String,
    pub starting_mileage: i64,
    pub end_mileage: Option<i32>,
    pub release_time: String,
    pub handover_time: Option<String>,
    pub guarantee_type: Option<String>,
    pub guarantee_property: Option<String>,
    pub customer_image: Vec<u8>,
    pub total_distance: Option<i64>,
    pub total_amount: Option<i32>,
    pub advanced_payment: Option<i64>,
    pub payment_type: Option<String>,
    pub payment_status: Option<String>,
    pub paid_amount: i64,
    pub order_status: Option<String>,
    pub notes: Option<String>,
    pub bank_account_name: Option<String>,
    pub bank_transfer_amount: Option<i64>,
    pub cash_amount: Option<i64>,
    pub discount: Option<i64>,
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOrderByOrderId {
    pub id: i64,
    pub order_number: String,
    pub customer_id: String,
    pub customer_name: String,
    pub contact_no: String,
    pub license_number: String,
    pub manufacturer: String,
    pub model_name: String,
    pub vehicle_register_number: String,
    pub starting_mileage: Option<i64>,
    pub release_time: String,
    pub end_mileage: Option<i64>,
    pub handover_time: Option<String>,
    pub total_amount: i64,
    pub order_status: String,
    pub created_at: String,
    pub customer_image_with_vehicle: Option<Vec<u8>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOrderResponse {
    pub order_number: String,
    pub customer_id: String,
    pub customer_image: Option<Vec<u8>>,
    pub vehicle_register_number: String,
    pub handover_time: Option<String>,
    pub release_time: Option<String>,
    pub order_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerOrder {
    pub id: i32,
    pub order_number: String,
    pub order_status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GetOrdersRequest {
    pub vehicle_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub date: Option<String>,           // YYYY-MM-DD format
    pub start_date: Option<String>,     // YYYY-MM-DD format
    pub limit: Option<i64>,             // Optional pagination limit
    pub offset: Option<i64>,
}
