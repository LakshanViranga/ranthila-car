use crate::models::customer::{CreateCustomerRequest, Customer, AddRestrictedCustomer, UpdateCustomerRequest};
use crate::AppState;
use crate::utils::common::{save_image, get_image};
use rusqlite::params;

#[tauri::command]
pub fn create_customer(state: tauri::State<AppState>, request: CreateCustomerRequest) -> Result<String, String> {
    //image saving
    let image_dir = state
            .app_dir
            .join("image")
            .join("customer");

    let front_image_path = save_image(request.license_front_image, &image_dir)?;
    let back_image_path =  save_image(request.license_back_image, &image_dir)?;

    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO customer (customer_id, customer_name, identity_number, license_number, license_front_image, license_back_image, address, contact_no, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            request.customer_id,
            request.customer_name,
            request.identity_number,
            request.license_number,
            &front_image_path,
            &back_image_path,
            request.address,
            request.contact_no,
            request.created_by,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("customer is added".to_string())
}

#[tauri::command]
pub fn get_customer_by_identity_number(state: tauri::State<AppState>, national_id: String) -> Result<Vec<Customer>, String> {

    let image_dir = state
                .app_dir
                .join("image")
                .join("customer");

    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare(
            "SELECT id, customer_id, customer_name, identity_number, license_number,
                    license_front_image, license_back_image, EXISTS ( SELECT 1 FROM restricted_customer rc WHERE rc.customer_id = c.customer_id
                        AND rc.is_active = 1
              ) AS is_blacklisted, address, contact_no, created_by, created_at
             FROM customer c WHERE identity_number = $1 ",
        )
        .map_err(|e| e.to_string())?;

    let customer = stmt
        .query_map([national_id], |row| {
            Ok(Customer {
                    id: row.get(0)?,
                    customer_id: row.get(1)?,
                    customer_name: row.get(2)?,
                    identity_number: row.get(3)?,
                    license_number: row.get(4)?,
                    license_front_image: get_image(row.get(5)?, &image_dir).ok(),
                    license_back_image: get_image(row.get(6)?, &image_dir).ok(),
                    is_blacked_listed: row.get(7)?,
                    address: row.get(8)?,
                    contact_no: row.get(9)?,
                    created_by: row.get(10)?,
                    created_at: row.get(11)?,
                }
             )
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Customer>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(customer)
}

#[tauri::command]
pub fn add_restricted_customer(state: tauri::State<AppState>,request: AddRestrictedCustomer) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;

    tx.execute("INSERT INTO restricted_customer (customer_id, reason, note, black_listed_date, created_by)
        VALUES (?1, ?2, ?3, ?4, ?5)",
       params![
            request.customer_id,
            request.reason,
            request.note,
            request.black_listed_date,
            request.created_by,
        ],)
    .map_err(|e| format!("Failed to add restricted customer: {}", e))?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok("Restricted customer added successfully".to_string())
}

#[tauri::command]
pub fn remove_restricted_customer(
    state: tauri::State<AppState>,
    nic: String,
) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;

    tx.execute("UPDATE restricted_customer SET is_active = 0 WHERE customer_id = ?1
              AND is_active = 1", params![nic],
        ).map_err(|e| format!("Failed to remove restricted customer: {}", e))?;
    tx.commit().map_err(|e| e.to_string())?;

    Ok("Customer removed from blacklist successfully".to_string())
}

#[tauri::command]
pub fn update_customer(state: tauri::State<AppState>, request: UpdateCustomerRequest) -> Result<String, String> {
    //image saving
    let image_dir = state.app_dir
                .join("image")
                .join("customer");

    let front_image_path = save_image(request.license_front_image, &image_dir)?;
    let back_image_path =  save_image(request.license_back_image, &image_dir)?;

    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Updated customer
    tx.execute(
        "UPDATE customer SET customer_name = ?1, license_number = ?2,
        license_front_image = ?3, license_back_image = ?4, address = ?5, contact_no = ?6
         WHERE customer_id = ?7",
        params![
            request.customer_name,
            request.license_number,
            &front_image_path,
            &back_image_path,
            request.address,
            request.contact_no,
            request.customer_id
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("Updated customer".to_string())
}
