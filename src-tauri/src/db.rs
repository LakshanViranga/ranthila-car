use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use crate::utils::common::{hash_password};


pub fn init_db(db_path: &PathBuf) -> Result<Connection, String> {
    let conn = Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", []).map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

    // Create tables
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS vehicles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vehicle_id TEXT NOT NULL,
            manufacturer TEXT NOT NULL,
            model_name TEXT NOT NULL,
            register_number TEXT NOT NULL UNIQUE,
            owner TEXT,
            fuel_type TEXT NOT NULL,
            transmission_type TEXT NOT NULL,
            mileage INTEGER NOT NULL,
            base_price INTEGER NOT NULL,
            unit_price INTEGER NOT NULL,
            addition_hour_price INTEGER NOT NULL,
            distance_range INTEGER NOT NULL DEFAULT 200,
            revenue_licence_date DATE,
            insurance_date DATE,
            created_by TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            is_deleted DATETIME DEFAULT NULL
        );

        -- ===== ORDERS (Main order record) =====
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_number TEXT NOT NULL UNIQUE,      -- INV-001, INV-002, etc.
            customer_id INTEGER NOT NULL,        -- NULL for walk-in customers
            vehicle_id TEXT NOT NULL,
            starting_mileage INTEGER NOT NULL,
            end_mileage INTEGER,
            release_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            handover_time DATETIME,
            guarantee_type TEXT,
            guarantee_property TEXT,
            customer_image TEXT,
            contact_no TEXT,
            total_distance INTEGER NOT NULL DEFAULT 0,
            total_amount INTEGER,              -- Final amount
            advanced_payment INTEGER,
            payment_type TEXT,
            payment_status TEXT DEFAULT 'completed',
            paid_amount INTEGER,
            order_status TEXT DEFAULT 'completed',   -- 'pending', 'completed', 'cancelled'
            notes TEXT,                              -- Special instructions
            bank_name TEXT,
            bank_transfer_amount INTEGER,
            cash_amount INTEGER,
            discount INTEGER,
            created_by TEXT NOT NULL,
            updated_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            is_deleted DATETIME
        );

        -- ===== CUSTOMER DETAILS (Customer details) =====
        CREATE TABLE IF NOT EXISTS customer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_id TEXT NOT NULL,
            customer_name TEXT NOT NULL,
            identity_number TEXT NOT NULL,
            license_number TEXT NOT NULL,
            license_front_image TEXT NOT NULL,
            license_back_image TEXT NOT NULL,
            is_blacked_listed BOOLEAN DEFAULT false,
            address TEXT,
            contact_no TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- ==== MAINTENANCE DETAILS TABLE =====
        CREATE TABLE IF NOT EXISTS maintenance (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vehicle_id TEXT NOT NULL,
            service_type TEXT NOT NULL,
            description TEXT NOT NULL,
            cost INTEGER NOT NULL,
            service_provider TEXT NOT NULL,
            service_date DATE,
            next_service_by TEXT,
            service_mileage INTEGER,
            next_service_date DATE,
            next_service_mileage INTEGER,
            payment_status TEXT,
            payment_type TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            is_deleted DATETIME DEFAULT NULL
        );

        -- ==== EXPENSES TABLE =====
        CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            date DATE,
            amount INTEGER NOT NULL,
            payment_type TEXT,
            payment_status TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            is_deleted DATETIME DEFAULT NULL
        );

        -- === ACCOUNT RECORDS TABLE ===
        CREATE TABLE IF NOT EXISTS account_summery (
            account_date DATE PRIMARY KEY,
            income_cash INTEGER,
            income_credit INTEGER,
            income_bank_transfer INTEGER,
            expense_cash INTEGER,
            expense_credit INTEGER,
            expense_bank INTEGER,
            bank_deposit INTEGER,
            hand_on_cash INTEGER,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- === INCIDENT MANAGEMENT TABLE ===
        CREATE TABLE IF NOT EXISTS incidents (
            id INTEGER PRIMARY KEY,
            order_number TEXT,
            customer_id TEXT,
            incident_type TEXT,
            severity TEXT,
            description TEXT,
            estimation_cost INTEGER,
            incident_date DATE,
            status TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- === TRANSACTIONS DETAILS TABLE ===
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            order_number TEXT,
            vehicle_id TEXT,
            amount INTEGER,
            payment_type TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- === USER DETAILS TABLE ====
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'USER',
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- === BLACK LISTED CUSTOMER TABLE ===
        CREATE TABLE IF NOT EXISTS restricted_customer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_id TEXT NOT NULL,
            reason TEXT NOT NULL,
            note TEXT,
            black_listed_date DATE,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATE DEFAULT CURRENT_TIMESTAMP,
            created_by TEXT
        );

        -- === BANK RECORDS ===
        CREATE TABLE IF NOT EXISTS bank_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            bank_name TEXT NOT NULL,
            amount INTEGER,
            deposit_date DATE,
            note TEXT,
            created_at DATE DEFAULT CURRENT_TIMESTAMP,
            created_by TEXT
        );

        -- ==== ORDER SEQUENCE ======
        CREATE TABLE IF NOT EXISTS order_sequence (
            seq_date DATE PRIMARY KEY,
            counter INTEGER NOT NULL
        );
        "
    ).map_err(|e| format!("Failed to create tables: {}", e))?;

    let password_hash = hash_password("admin")?;

    // Check if users table already exists
    let admin_users_exists: bool = conn
        .query_row(
            r#"
            SELECT EXISTS(
                SELECT *
                FROM users
                WHERE role = 'ADMIN'
                AND username = 'admin'
            )
            "#,
            [],
            |row| row.get(0),
            )
        .map_err(|e| format!("Failed to check users table: {}", e))?;

    if !admin_users_exists {
        conn.execute(
                r#"
                INSERT INTO users (username, password, role, status)
                VALUES (?1, ?2, ?3, ?4)
                "#,
                params![
                    "admin",
                    password_hash,
                    "ADMIN",
                    "ACTIVE"
                ],
        ).map_err(|e| format!("Failed to create default user: {}", e))?;
    }

    Ok(conn)
}
