use crate::models::order::{
    CreateOrderRequest, CustomerOrder, FilterOrderResponse, GetOrderByOrderId, GetOrderCount,
    GetOrderFilterRequest, Order, UpdateOrderRequest,
};
use crate::utils::common::{get_image, save_image};
use crate::AppState;
use chrono::Local;
use rusqlite::params;

#[tauri::command]
pub fn create_order(
    state: tauri::State<AppState>,
    request: CreateOrderRequest,
) -> Result<String, String> {
    // Inserting image
    let image_dir = state.app_dir.join("image").join("order");
    let order_image_path = if request.customer_image.is_empty() {
        "not_set".to_string()
    } else {
        save_image(request.customer_image, &image_dir)?
    };

    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "INSERT INTO orders (order_number, customer_id, vehicle_id, starting_mileage, release_time, handover_time, guarantee_type, guarantee_property,
            customer_image, contact_no, total_amount, advanced_payment, payment_type, payment_status, paid_amount,
            order_status, notes, created_by) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![
            request.order_number,
            request.customer_id,
            request.vehicle_id,
            request.starting_mileage,
            request.release_time,
            request.handover_time,
            request.guarantee_type,
            request.guarantee_property,
            &order_image_path,
            request.contact_no,
            request.total_amount,
            request.advanced_payment,
            request.payment_type,
            request.payment_status,
            request.paid_amount,
            request.order_status,
            request.notes,
            request.created_by
        ],
    ).map_err(|e| {
        println!("{}", e);
        e.to_string()
    })?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("order created".to_string())
}

#[tauri::command]
pub fn get_orders(state: tauri::State<AppState>) -> Result<Vec<Order>, String> {
    let db = state.db.lock().unwrap();

    let image_dir = state.app_dir.join("image").join("order");

    let mut stmt = db
        .prepare(
            "SELECT o.id, o.order_number, o.customer_id, o.vehicle_id, o.starting_mileage, o.end_mileage,
                    o.release_time, o.handover_time, o.guarantee_property, o.guarantee_type, o.customer_image, o.total_distance,
                    o.total_amount, o.advanced_payment, o.payment_type, o.payment_status, o.order_status, o.notes, o.created_by, o.created_at, o.updated_by, o.updated_at,
                    c.customer_name, o.paid_amount, o.contact_no, o.bank_name, o.bank_transfer_amount, o.cash_amount, o.discount
             FROM orders o JOIN customer c ON o.customer_id = c.customer_id WHERE is_deleted IS NULL ORDER BY o.created_at DESC LIMIT 100",
        )
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            e.to_string()
         })?;

    let orders = stmt
        .query_map([], |row| {
            Ok(Order {
                id: row.get(0)?,
                order_number: row.get(1)?,
                customer_id: row.get(2)?,
                vehicle_id: row.get(3)?,
                starting_mileage: row.get(4)?,
                end_mileage: row.get(5)?,
                release_time: row.get(6)?,
                handover_time: row.get(7)?,
                guarantee_property: row.get(8)?,
                guarantee_type: row.get(9)?,
                customer_image: if row.get::<_, String>(10)? == "not_set" {
                    None
                } else {
                    get_image(row.get(10)?, &image_dir).ok()
                },
                total_distance: row.get(11)?,
                total_amount: row.get(12)?,
                advanced_payment: row.get(13)?,
                payment_type: row.get(14)?,
                payment_status: row.get(15)?,
                order_status: row.get(16)?,
                notes: row.get(17)?,
                created_by: row.get(18)?,
                created_at: row.get(19)?,
                updated_by: row.get(20)?,
                updated_at: row.get(21)?,
                customer_name: row.get(22)?,
                paid_amount: row.get(23)?,
                contact_no: row.get(24)?,
                bank_account_name: row.get(25)?,
                bank_transfer_amount: row.get(26)?,
                cash_amount: row.get(27)?,
                discount: row.get(28)?,
            })
        })
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            e.to_string()
        })?
        .collect::<Result<Vec<Order>, _>>()
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            e.to_string()
        })?;

    Ok(orders)
}

#[tauri::command]
pub fn get_order_sequence(state: tauri::State<AppState>) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    tx.execute(
             "INSERT INTO order_sequence (seq_date, counter) VALUES (date('now'), 1) ON CONFLICT(seq_date)
             DO UPDATE SET counter = counter + 1",
             [],
         )
         .map_err(|e| e.to_string())?;

    // read counter
    let counter = tx
        .query_row(
            "SELECT counter FROM order_sequence WHERE seq_date = date('now')",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    let date = Local::now().format("%Y%m%d").to_string();
    Ok(format!("{}-{:05}", date, counter))
}

#[tauri::command]
pub fn update_order(
    state: tauri::State<AppState>,
    request: UpdateOrderRequest,
) -> Result<String, String> {
    // Inserting image
    let image_dir = state.app_dir.join("image").join("order");

    let order_image_path = if request.customer_image.is_empty() {
        "not_set".to_string()
    } else {
        save_image(request.customer_image, &image_dir)?
    };

    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "UPDATE orders SET customer_id = ?1, vehicle_id = ?2, starting_mileage = ?3, end_mileage = ?4,
         release_time = ?5, handover_time = ?6, guarantee_type = ?7, guarantee_property = ?8, customer_image = ?9, total_distance = ?10, total_amount = ?11, advanced_payment = ?12,
         payment_type = ?13, payment_status = ?14, paid_amount = ?15, order_status = ?16, notes = ?17, updated_by = ?18,  bank_name = ?19, bank_transfer_amount = ?20, cash_amount = ?21, discount = ?22
         WHERE order_number = ?23",
        params![
            request.customer_id,
            request.vehicle_id,
            request.starting_mileage,
            request.end_mileage,
            request.release_time,
            request.handover_time,
            request.guarantee_type,
            request.guarantee_property,
            &order_image_path,
            request.total_distance,
            request.total_amount,
            request.advanced_payment,
            request.payment_type,
            request.payment_status,
            request.paid_amount,
            request.order_status,
            request.notes,
            request.updated_by,
            request.bank_account_name,
            request.bank_transfer_amount,
            request.cash_amount,
            request.discount,
            request.order_number,
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
pub fn delete_order(state: tauri::State<AppState>, id: i64) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "UPDATE orders SET is_deleted = CURRENT_TIMESTAMP
         WHERE id = ?1",
        params![id,],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("order deleted".to_string())
}

#[tauri::command]
pub fn get_customer_order(
    state: tauri::State<AppState>,
    national_id: String,
) -> Result<Vec<CustomerOrder>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, order_number, order_status, created_at
            FROM orders WHERE customer_id = $1 AND is_deleted IS NULL ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let customer_order = stmt
        .query_map([national_id], |row| {
            Ok(CustomerOrder {
                id: row.get(0)?,
                order_number: row.get(1)?,
                order_status: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<CustomerOrder>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(customer_order)
}

#[tauri::command]
pub fn get_order_by_order_number(
    state: tauri::State<AppState>,
    order_id: String,
) -> Result<Vec<GetOrderByOrderId>, String> {
    let image_dir = state.app_dir.join("image").join("order");

    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare(
            "SELECT o.id, o.order_number, o.customer_id, c.customer_name, o.contact_no, c.license_number,
             v.manufacturer, v.model_name, v.register_number, o.starting_mileage, o.release_time,
             o.end_mileage, o.handover_time, o.total_amount, o.order_status, o.created_at, o.customer_image
            FROM orders o JOIN customer c ON o.customer_id = c.customer_id JOIN vehicles v ON o.vehicle_id = v.vehicle_id WHERE o.order_number = $1 ORDER BY o.created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let get_order = stmt
        .query_map([order_id], |row| {
            Ok(GetOrderByOrderId {
                id: row.get(0)?,
                order_number: row.get(1)?,
                customer_id: row.get(2)?,
                customer_name: row.get(3)?,
                contact_no: row.get(4)?,
                license_number: row.get(5)?,
                manufacturer: row.get(6)?,
                model_name: row.get(7)?,
                vehicle_register_number: row.get(8)?,
                starting_mileage: row.get(9)?,
                release_time: row.get(10)?,
                end_mileage: row.get(11)?,
                handover_time: row.get(12)?,
                total_amount: row.get(13)?,
                order_status: row.get(14)?,
                created_at: row.get(15)?,
                customer_image_with_vehicle: get_image(row.get(16)?, &image_dir).ok(),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<GetOrderByOrderId>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(get_order)
}

#[tauri::command]
pub fn filter_order(
    state: tauri::State<AppState>,
    request: GetOrderFilterRequest,
) -> Result<Vec<FilterOrderResponse>, String> {
    let image_dir = state.app_dir.join("image").join("order");

    let db = state.db.lock().unwrap();

    // Count how many filters are provided (should be at most 1)
    let filter_count = [
        request.vehicle_id.is_some(),
        request.customer_id.is_some(),
        request.created_date.is_some(),
        request.start_date.is_some(),
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    if filter_count > 1 {
        return Err("Only one search parameter is allowed at a time".to_string());
    }

    let mut query = "SELECT o.order_number, o.customer_id, o.release_time, o.handover_time, o.customer_image, o.order_status,
                     v.register_number FROM orders o JOIN vehicles v ON o.vehicle_id = v.vehicle_id WHERE o.is_deleted IS NULL".to_string();

    // Add WHERE clause based on provided filter
    if let Some(vehicle_id) = request.vehicle_id {
        query.push_str(&format!(" AND o.vehicle_id = '{}'", vehicle_id));
    } else if let Some(customer_id) = request.customer_id {
        query.push_str(&format!(" AND o.customer_id = '{}'", customer_id));
    } else if let Some(created_date) = request.created_date {
        // Date range: from start of day to start of next day
        query.push_str(&format!(" AND DATE(o.created_at) = '{}'", created_date));
    } else if let Some(start_date) = request.start_date {
        // From start_date onwards
        query.push_str(&format!(" AND DATE(o.created_at) >= '{}'", start_date));
    }

    // Add sorting
    query.push_str(" ORDER BY o.created_at DESC");

    // Add pagination
    let offset = request.offset.unwrap_or(0);
    if let Some(limit) = request.limit {
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    }

    let mut stmt = db
        .prepare(&query)
        .map_err(|e| format!("Query error: {}", e))?;

    let orders = stmt
        .query_map([], |row| {
            Ok(FilterOrderResponse {
                order_number: row.get(0)?,
                customer_id: row.get(1)?,
                release_time: row.get(2)?,
                handover_time: row.get(3)?,
                customer_image: get_image(row.get(4)?, &image_dir).ok(),
                order_status: row.get(5)?,
                vehicle_register_number: row.get(6)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row mapping error: {}", e))?;
    Ok(orders)
}

// Get order count
#[tauri::command]
pub fn get_order_count_by_customer_id(
    state: tauri::State<AppState>,
    customer_id: String,
) -> Result<GetOrderCount, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare("SELECT count(*) FROM orders  WHERE is_deleted IS NULL AND customer_id = ?1")
        .map_err(|e| format!("Query error: {}", e))?;
    let count: i64 = stmt
        .query_row([customer_id.clone()], |row| row.get(0))
        .map_err(|e| format!("Query error: {}", e))?;
    Ok(GetOrderCount {
        customer_id: customer_id.clone(),
        count,
    })
}
