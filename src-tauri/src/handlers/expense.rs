use crate::models::expense::{AddExpensesRecord, ExpensesRecord, UpdateExpensesRecord, UpdatePaymentStatus};
use crate::AppState;
use rusqlite::params;

// Add expenses
#[tauri::command]
pub fn add_expenses(state: tauri::State<AppState>, request: AddExpensesRecord) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
        let tx = db.transaction().map_err(|e| e.to_string())?;
        // Insert order
        tx.execute(
            "INSERT INTO expenses (category, description, date, amount, payment_type, payment_status, created_by)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                request.category,
                request.description,
                request.date,
                request.amount,
                request.payment_type,
                request.payment_status,
                request.created_by,
            ],
        )
        .map_err(|e| e.to_string())?;

        tx.commit().map_err(|e| e.to_string())?;

        Ok("Expenses record is added".to_string())
}

// Get expenses record
#[tauri::command]
pub fn get_expenses(state: tauri::State<AppState>) -> Result<Vec<ExpensesRecord>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, category, description, date, amount, payment_type, payment_status,
            created_by, created_at FROM expenses WHERE is_deleted IS NULL ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let maintenance = stmt
        .query_map([], |row| {
            Ok(ExpensesRecord {
                id: row.get(0)?,
                category: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
                amount: row.get(4)?,
                payment_type: row.get(5)?,
                payment_status: row.get(6)?,
                created_by: row.get(7)?,
                created_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<ExpensesRecord>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(maintenance)
}

// Delete expenses record
#[tauri::command]
pub fn delete_expenses(state: tauri::State<AppState>, id: i64) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;

    // Delete expenses record
    tx.execute("UPDATE expenses SET is_deleted = CURRENT_TIMESTAMP WHERE id = ?1", params![id],)
        .map_err(|e| {e.to_string()})?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Delete expenses record success".to_string())
}

// Update expenses record
#[tauri::command]
pub fn update_expenses(state: tauri::State<AppState>, request: UpdateExpensesRecord) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();

    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Edit Vehicle
    tx.execute(
       "UPDATE expenses SET category = ?1, description = ?2, amount = ?3, date = ?4, payment_type = ?5, payment_status = ?6
       WHERE id = ?7",
       params![
           request.category,
           request.description,
           request.amount,
           request.date,
           request.payment_type,
           request.payment_status,
           request.id
       ],
    ).map_err(|e| {e.to_string()})?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("expenses record is updated".to_string())
}

// Complete expenses record
#[tauri::command]
pub fn complete_expenses(state: tauri::State<AppState>, request: UpdatePaymentStatus) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
        let tx = db.transaction().map_err(|e| e.to_string())?;
        // Edit Vehicle
        tx.execute(
            "UPDATE expenses SET payment_status = ?1 WHERE id = ?2",
            params![
                request.payment_status,
                request.id,
            ],
        )
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            e.to_string()
            })?;

        tx.commit().map_err(|e| e.to_string())?;

        Ok("order status updated".to_string())
}
