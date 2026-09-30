use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i32,
    pub customer_id: String,
    pub customer_name: String,
    pub identity_number: String,
    pub license_number: String,
    pub license_front_image: Option<Vec<u8>>,
    pub license_back_image: Option<Vec<u8>>,
    pub is_blacked_listed: bool,
    pub address: String,
    pub contact_no: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCustomerRequest {
    pub customer_id: String,
    pub customer_name: String,
    pub identity_number: String,
    pub license_number: String,
    pub license_front_image: Vec<u8>,
    pub license_back_image: Vec<u8>,
    pub address: String,
    pub contact_no: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRestrictedCustomer {
    pub customer_id: String,
    pub reason: String,
    pub note: Option<String>,
    pub black_listed_date: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCustomerRequest {
    pub customer_id: String,
    pub customer_name: String,
    pub license_number: String,
    pub license_front_image: Vec<u8>,
    pub license_back_image: Vec<u8>,
    pub address: String,
    pub contact_no: String,
}
