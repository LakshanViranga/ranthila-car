use crate::models::maintenance::{
    AddMaintenanceRequest, MaintenanceRecord, UpdateMaintenanceRequest, UpdatePaymentStatus,
};
use crate::AppState;
use rusqlite::params;

// Add maintenance record
#[tauri::command]
pub fn add_maintenance_record(
    state: tauri::State<AppState>,
    request: AddMaintenanceRequest,
) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO maintenance (vehicle_id, service_type, service_date, description, cost, service_provider, service_mileage, next_service_by, next_service_date, next_service_mileage, payment_status, payment_type, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            request.vehicle_id,
            request.service_type,
            request.service_date,
            request.description,
            request.cost,
            request.service_provider,
            request.service_mileage,
            request.next_service_by,
            request.next_service_date,
            request.next_service_mileage,
            request.payment_status,
            request.payment_type,
            request.created_by,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Maintenance record is added".to_string())
}

#[tauri::command]
pub fn get_maintenance_record(
    state: tauri::State<AppState>,
) -> Result<Vec<MaintenanceRecord>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT m.id, m.vehicle_id, v.register_number, m.service_type, m.service_date, m.description, m.cost, m.service_provider, m.service_mileage, m.next_service_date, m.next_service_mileage,
            m.payment_status, m.payment_type, m.created_by, m.created_at, m.next_service_by FROM maintenance m JOIN vehicles v ON m.vehicle_id = v.vehicle_id WHERE m.is_deleted IS NULL  ORDER BY m.created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let maintenance = stmt
        .query_map([], |row| {
            Ok(MaintenanceRecord {
                id: row.get(0)?,
                vehicle_id: row.get(1)?,
                vehicle_register_number: row.get(2)?,
                service_type: row.get(3)?,
                service_date: row.get(4)?,
                description: row.get(5)?,
                cost: row.get(6)?,
                service_provider: row.get(7)?,
                service_mileage: row.get(8)?,
                next_service_date: row.get(9)?,
                next_service_mileage: row.get(10)?,
                payment_status: row.get(11)?,
                payment_type: row.get(12)?,
                created_by: row.get(13)?,
                created_at: row.get(14)?,
                next_service_by: row.get(15)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<MaintenanceRecord>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(maintenance)
}

// Delete maintenance record
#[tauri::command]
pub fn delete_maintenance(state: tauri::State<AppState>, id: i64) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;

    // Delete maintenance record
    tx.execute(
        "UPDATE maintenance SET is_deleted = CURRENT_TIMESTAMP WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Delete maintenance record success".to_string())
}

// Update maintenance record
#[tauri::command]
pub fn update_maintenance(
    state: tauri::State<AppState>,
    request: UpdateMaintenanceRequest,
) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();

    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Edit Vehicle
    tx.execute(
       "UPDATE maintenance SET service_type = ?1, description = ?2, service_provider = ?3, service_date = ?4, service_mileage = ?5, next_service_date = ?6, next_service_mileage = ?7, next_service_by = ?8
       WHERE id = ?9",
       params![
           request.service_type,
           request.description,
           request.service_provider,
           request.service_date,
           request.service_mileage,
           request.next_service_date,
           request.next_service_mileage,
           request.next_service_by,
           request.id
       ],
    ).map_err(|e| {e.to_string()})?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("maintenance record is updated".to_string())
}

// Complete maintenance record
#[tauri::command]
pub fn complete_maintenance(
    state: tauri::State<AppState>,
    request: UpdatePaymentStatus,
) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();

    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Edit Vehicle
    tx.execute(
        "UPDATE maintenance SET payment_status = ?1 WHERE id = ?2",
        params![request.payment_status, request.id,],
    )
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        e.to_string()
    })?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("order status updated".to_string())
}
