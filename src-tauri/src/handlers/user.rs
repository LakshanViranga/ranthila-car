use crate::models::user::{LoginRequest, LoginResponse, SaveUserRequest, User, UserDb};
use crate::utils::common::{hash_password, verify_password};
use crate::AppState;
use rusqlite::params;

#[tauri::command]
pub fn get_users(state: tauri::State<AppState>) -> Result<Vec<User>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare(
            "SELECT id, username, role, status, created_at FROM users ORDER BY created_at DESC",
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let users = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                role: row.get(2)?,
                status: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {}", e))?;
    Ok(users)
}

#[tauri::command]
pub fn save_user(
    state: tauri::State<AppState>,
    request: SaveUserRequest,
) -> Result<String, String> {
    let hashed_password = hash_password(&request.password)?;

    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert user
    tx.execute(
        "INSERT INTO users (username, password, role, status)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            request.username,
            hashed_password,
            request.role,
            request.status,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("User record is added".to_string())
}

#[tauri::command]
pub fn login(
    state: tauri::State<AppState>,
    request: LoginRequest,
) -> Result<LoginResponse, String> {
    if request.username.is_empty() || request.password.is_empty() {
        return Ok(LoginResponse {
            success: false,
            user: None,
            message: "Username and password are required".to_string(),
        });
    }

    let db = state.db.lock().unwrap();
    let result: Result<UserDb, _> = db.query_row("SELECT id, username, password, role, status, created_at FROM users WHERE LOWER(username) = ?1",
        params![request.username],
        |row| {
            Ok(UserDb {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                role: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
            })
        },
    );

    match result {
        Ok(user_db) => {
            // Verify password
            match verify_password(&request.password, &user_db.password) {
                Ok(true) => {
                    if user_db.status != "ACTIVE" {
                        return Ok(LoginResponse {
                            success: false,
                            user: None,
                            message: format!("User account is {}", user_db.status),
                        });
                    }
                    Ok(LoginResponse {
                        success: true,
                        user: Some(User {
                            id: user_db.id,
                            username: user_db.username,
                            role: user_db.role,
                            status: user_db.status,
                            created_at: user_db.created_at,
                        }),
                        message: "Login successful".to_string(),
                    })
                }
                Ok(false) => Ok(LoginResponse {
                    success: false,
                    user: None,
                    message: "Invalid username or password".to_string(),
                }),
                Err(e) => Ok(LoginResponse {
                    success: false,
                    user: None,
                    message: format!("Authentication error: {}", e),
                }),
            }
        }
        Err(_) => Ok(LoginResponse {
            success: false,
            user: None,
            message: "Invalid username or password".to_string(),
        }),
    }
}
