use crate::models::bank::{AddBankRecordRequest, BankRecord};
use crate::AppState;
use rusqlite::params;

#[tauri::command]
pub fn add_bank_record(state: tauri::State<AppState>, request: AddBankRecordRequest ) -> Result<String, String> {
    let mut db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    let tx = db
        .transaction()
        .map_err(|e| format!("Transaction error: {}", e))?;
    tx.execute(
        "INSERT INTO bank_records (bank_name, amount, deposit_date, note, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            request.bank_name,
            request.amount,
            request.deposit_date,
            request.note,
            request.created_by,
        ],
    )
    .map_err(|e| format!("Failed to insert record: {}", e))?;

    tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok("Bank record added successfully".to_string())
}

/// Get all bank records
#[tauri::command]
pub fn get_all_bank_records(state: tauri::State<AppState>) -> Result<Vec<BankRecord>, String> {
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;

    let mut stmt = db.prepare(
            "SELECT id, bank_name, amount, deposit_date, note, created_at, created_by
             FROM bank_records
             ORDER BY created_at DESC",
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let records = stmt
        .query_map([], |row| {
            Ok(BankRecord {
                id: row.get(0)?,
                bank_name: row.get(1)?,
                amount: row.get(2)?,
                deposit_date: row.get(3)?,
                note: row.get(4)?,
                created_at: row.get(5)?,
                created_by: row.get(6)?,
            })
        })
        .map_err(|e| format!("Row mapping error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect records: {}", e))?;

    Ok(records)
}


