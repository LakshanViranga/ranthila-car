use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateVehicleRequest {
    pub vehicle_id: String,
    pub manufacturer: String,
    pub model_name: String,
    pub register_number: String,
    pub owner: String,
    pub fuel_type: String,
    pub transmission_type: String,
    pub mileage: i32,
    pub base_price: i32,
    pub unit_price: i32,
    pub addition_hour_price: i32,
    pub distance_range: i64,
    pub revenue_licence_date: String,
    pub insurance_date: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vehicle {
    pub id: i32,
    pub vehicle_id: String,
    pub manufacturer: String,
    pub model_name: String,
    pub register_number: String,
    pub owner: String,
    pub fuel_type: String,
    pub transmission_type: String,
    pub mileage: i32,
    pub base_price: i32,
    pub unit_price: i32,
    pub addition_hour_price: i32,
    pub distance_range: i64,
    pub revenue_licence_date: String,
    pub insurance_date: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditVehicleRequest {
    pub vehicle_id: String,
    pub owner: String,
    pub base_price: i32,
    pub unit_price: i32,
    pub addition_hour_price: i32,
    pub distance_range: i64,
    pub revenue_licence_date: String,
    pub insurance_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateVehicleMileage {
    pub vehicle_id: String,
    pub mileage: i64,
}
