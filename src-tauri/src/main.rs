#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
mod handlers;
mod models;
mod utils;

use tauri_plugin_log::{Target, TargetKind};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::handlers::account::{add_account_summery, get_account_summery};
use crate::handlers::bank::{add_bank_record, get_all_bank_records};
use crate::handlers::customer::{
    add_restricted_customer, create_customer, get_customer_by_identity_number,
    remove_restricted_customer, update_customer,
};
use crate::handlers::expense::{
    add_expenses, complete_expenses, delete_expenses, get_expenses, update_expenses,
};
use crate::handlers::incident::{add_incident, get_incidents};
use crate::handlers::maintenance::{
    add_maintenance_record, complete_maintenance, delete_maintenance, get_maintenance_record,
    update_maintenance,
};
use crate::handlers::order::{
    create_order, delete_order, filter_order, get_customer_order, get_order_by_order_number,
    get_order_count_by_customer_id, get_order_sequence, get_orders, update_order,
};
use crate::handlers::transaction::{add_transaction, get_transaction};
use crate::handlers::user::{get_users, login, save_user};
use crate::handlers::vehicle::{
    create_vehicle, delete_vehicle, get_vehicles, update_vehicle, update_vehicle_mileage,
};

#[derive(Clone)]
pub struct AppState {
    db: std::sync::Arc<Mutex<Connection>>,
    app_dir: std::path::PathBuf,
}

fn main() {
    // Get app data directory (cross-platform compatible)
    let app_dir = get_app_data_dir();
    std::fs::create_dir_all(&app_dir).ok();

    // Initialize database
    let db_path = app_dir.join("rent_system.db");

    let conn = db::init_db(&db_path).expect("failed to initialize database");

    // Image directories
    let customer_image_dir = app_dir.join("image").join("customer");
    let order_image_dir = app_dir.join("image").join("order");

    std::fs::create_dir_all(&customer_image_dir)
        .expect("Failed to create customer image directory");

    std::fs::create_dir_all(&order_image_dir).expect("Failed to create order image directory");

    let state = AppState {
        app_dir,
        db: std::sync::Arc::new(Mutex::new(conn)),
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_order,
            create_vehicle,
            create_customer,
            get_orders,
            get_vehicles,
            update_vehicle,
            delete_vehicle,
            get_customer_by_identity_number,
            get_order_sequence,
            update_order,
            delete_order,
            delete_vehicle,
            add_maintenance_record,
            get_maintenance_record,
            update_maintenance,
            delete_maintenance,
            complete_maintenance,
            add_expenses,
            get_expenses,
            update_expenses,
            delete_expenses,
            complete_expenses,
            add_account_summery,
            get_account_summery,
            get_customer_order,
            get_incidents,
            add_incident,
            add_transaction,
            get_transaction,
            get_order_by_order_number,
            save_user,
            get_users,
            add_restricted_customer,
            remove_restricted_customer,
            filter_order,
            login,
            get_all_bank_records,
            add_bank_record,
            update_vehicle_mileage,
            get_order_count_by_customer_id,
            update_customer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Get app data directory - cross-platform
fn get_app_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let app_data = std::env::var("APPDATA")
            .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_default());
        PathBuf::from(app_data).join("restaurant-pos")
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        PathBuf::from(home).join("Library/Application Support/restaurant-pos")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        let xdg_data =
            std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| format!("{}/.local/share", home));
        PathBuf::from(xdg_data).join("restaurant-pos")
    }
}
