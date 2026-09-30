use crate::models::account::{AccountSummery, AddAccountSummery};
use crate::AppState;
use rusqlite::params;

// Add account summery table
#[tauri::command]
pub fn add_account_summery(
    state: tauri::State<AppState>,
    request: AddAccountSummery,
) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO account_summery (account_date, income_cash, income_credit, income_bank_transfer, expense_cash, expense_credit, expense_bank, bank_deposit, hand_on_cash, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
         ON CONFLICT(account_date) DO UPDATE SET
             income_cash = excluded.income_cash,
             income_credit = excluded.income_credit,
             income_bank_transfer = excluded.income_bank_transfer,
             expense_cash = excluded.expense_cash,
             expense_credit = excluded.expense_credit,
             expense_bank = excluded.expense_bank,
             bank_deposit = excluded.bank_deposit,
             hand_on_cash = excluded.hand_on_cash,
             created_by = excluded.created_by",
        params![
            request.date,
            request.income_cash,
            request.income_credit,
            request.income_bank_transfer,
            request.expenses_cash,
            request.expenses_credit,
            request.expenses_bank,
            request.bank_deposit,
            request.hand_on_cash,
            request.created_by,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Account Summery record is added".to_string())
}

#[tauri::command]
pub fn get_account_summery(state: tauri::State<AppState>) -> Result<Vec<AccountSummery>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT account_date, income_cash, income_credit, income_bank_transfer, expense_cash, expense_credit, expense_bank, bank_deposit, hand_on_cash, created_by, created_at
            FROM account_summery  ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let maintenance = stmt
        .query_map([], |row| {
            Ok(AccountSummery {
                date: row.get(0)?,
                income_cash: row.get(1)?,
                income_credit: row.get(2)?,
                income_bank_transfer: row.get(3)?,
                expenses_cash: row.get(4)?,
                expenses_credit: row.get(5)?,
                expenses_bank: row.get(6)?,
                bank_deposit: row.get(7)?,
                hand_on_cash: row.get(8)?,
                created_by: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<AccountSummery>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(maintenance)
}
