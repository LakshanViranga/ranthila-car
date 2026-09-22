use crate::models::transaction::{AddTransaction, Transaction};
use crate::AppState;
use rusqlite::params;

#[tauri::command]
pub fn add_transaction(state: tauri::State<AppState>, request: AddTransaction) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO transactions (order_number, vehicle_id, amount, payment_type, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            request.order_number,
            request.vehicle_id,
            request.amount,
            request.payment_type,
            request.created_by
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Incident record is added".to_string())
}

#[tauri::command]
pub fn get_transaction(state: tauri::State<AppState>) -> Result<Vec<Transaction>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, order_number, vehicle_id, amount, payment_type, created_by, created_at
            FROM transactions",
        )
        .map_err(|e| e.to_string())?;

    let transactions = stmt
        .query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                order_number: row.get(1)?,
                vehicle_id: row.get(2)?,
                amount: row.get(3)?,
                payment_type: row.get(4)?,
                created_by: row.get(5)?,
                created_at: row.get(6)?
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Transaction>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(transactions)
}
