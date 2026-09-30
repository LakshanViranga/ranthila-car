use crate::models::incident::{AddIncident, Incident};
use crate::AppState;
use rusqlite::params;

// Incident Handling
#[tauri::command]
pub fn add_incident(state: tauri::State<AppState>, request: AddIncident) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO incidents (order_number, customer_id, incident_type, severity, description, estimation_cost, incident_date, status, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            request.order_number,
            request.customer_id,
            request.incident_type,
            request.severity,
            request.description,
            request.estimation_cost,
            request.incident_date,
            request.status,
            request.created_by,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Incident record is added".to_string())
}

#[tauri::command]
pub fn get_incidents(
    state: tauri::State<AppState>,
    national_id: String,
) -> Result<Vec<Incident>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, order_number, customer_id, incident_type, severity, description, estimation_cost,
            incident_date, status, created_by, created_at FROM incidents WHERE customer_id = $1  ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let incidents = stmt
        .query_map([national_id], |row| {
            Ok(Incident {
                id: row.get(0)?,
                order_number: row.get(1)?,
                customer_id: row.get(2)?,
                incident_type: row.get(3)?,
                severity: row.get(4)?,
                description: row.get(5)?,
                estimation_cost: row.get(6)?,
                incident_date: row.get(7)?,
                status: row.get(8)?,
                created_by: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Incident>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(incidents)
}
