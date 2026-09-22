use crate::models::vehicle::{CreateVehicleRequest, Vehicle, EditVehicleRequest, UpdateVehicleMileage};
use crate::AppState;
use rusqlite::params;

#[tauri::command]
pub fn create_vehicle(state: tauri::State<AppState>, request: CreateVehicleRequest) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO vehicles (vehicle_id, manufacturer, model_name, register_number, owner, fuel_type, transmission_type,
            mileage, base_price, unit_price, addition_hour_price, distance_range, revenue_licence_date, insurance_date, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            request.vehicle_id ,
            request.manufacturer,
            request.model_name,
            request.register_number,
            request.owner,
            request.fuel_type,
            request.transmission_type,
            request.mileage,
            request.base_price,
            request.unit_price,
            request.addition_hour_price,
            request.distance_range,
            request.revenue_licence_date,
            request.insurance_date,
            request.created_by
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("vehicle is added".to_string())
}

#[tauri::command]
pub fn get_vehicles(state: tauri::State<AppState>) -> Result<Vec<Vehicle>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, vehicle_id, manufacturer, model_name, register_number,owner,fuel_type, transmission_type, mileage,
            base_price, unit_price, addition_hour_price, distance_range, revenue_licence_date, insurance_date, created_by, created_at
             FROM vehicles WHERE is_deleted IS NULL ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let products = stmt
        .query_map([], |row| {
            Ok(Vehicle {
                id: row.get(0)?,
                vehicle_id: row.get(1)?,
                manufacturer: row.get(2)?,
                model_name: row.get(3)?,
                register_number: row.get(4)?,
                owner: row.get(5)?,
                fuel_type: row.get(6)?,
                transmission_type: row.get(7)?,
                mileage: row.get(8)?,
                base_price: row.get(9)?,
                unit_price: row.get(10)?,
                addition_hour_price: row.get(11)?,
                distance_range: row.get(12)?,
                revenue_licence_date: row.get(13)?,
                insurance_date: row.get(14)?,
                created_by: row.get(15)?,
                created_at: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Vehicle>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(products)
}

// Edit vehicle
#[tauri::command]
pub fn update_vehicle(state: tauri::State<AppState>, request: EditVehicleRequest) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Edit Vehicle
    tx.execute(
        "UPDATE vehicles SET owner = ?1, base_price = ?2, unit_price = ?3, addition_hour_price = ?4, distance_range = ?5,
         revenue_licence_date = ?6, insurance_date = ?7,
         WHERE vehicle_id = ?8",
        params![
            request.owner,
            request.base_price,
            request.unit_price,
            request.addition_hour_price,
            request.vehicle_id,
            request.distance_range,
            request.revenue_licence_date,
            request.insurance_date
        ],
    )
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e.to_string()
        })?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("order status updated".to_string())
}

#[tauri::command]
pub fn delete_vehicle(state: tauri::State<AppState>, vehicle_id: String) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "UPDATE vehicles SET is_deleted = CURRENT_TIMESTAMP
         WHERE vehicle_id = ?1",
        params![
            vehicle_id,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("vehicle deleted".to_string())
}

// Update Vehicle Mileage
#[tauri::command]
pub fn update_vehicle_mileage(state: tauri::State<AppState>, request: UpdateVehicleMileage) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Update Vehicle Mileage
    tx.execute(
        "UPDATE vehicles SET mileage = ?1 WHERE vehicle_id = ?2",
        params![
            request.mileage,
            request.vehicle_id,
        ],
    )
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e.to_string()
        })?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Vehicle Mileage Updated".to_string())
}
