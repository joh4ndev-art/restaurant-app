mod commands;
mod domain;
mod error;
mod infrastructure;
mod services;
mod state;

use std::fs;

use tauri::Manager;

use crate::infrastructure::db;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("restaurant.db");

            let conn = db::connection::open(&db_path)?;
            db::migrations::run(&conn)?;

            // Daily automatic backup (runs at most once per day).
            // Failures are logged but never block startup.
            if let Err(e) = crate::services::BackupService::run_auto_if_needed(app.handle(), &conn) {
                eprintln!("auto-backup failed: {e}");
            }

            app.manage(AppState::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tables::list_tables,
            commands::tables::create_table,
            commands::tables::update_table_status,
            commands::tables::delete_table,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::categories::list_categories,
            commands::categories::create_category,
            commands::categories::delete_category,
            commands::products::list_products,
            commands::products::create_product,
            commands::products::update_product,
            commands::products::delete_product,
            commands::orders::send_order,
            commands::orders::list_open_orders,
            commands::orders::get_order,
            commands::orders::cancel_order,
            commands::orders::get_active_order_for_table,
            commands::orders::mark_order_served,
            commands::orders::list_orders_by_status,
            commands::payments::pay_order,
            commands::payments::get_payment_summary,
            commands::reports::generate_report,
            commands::users::has_users,
            commands::users::login,
            commands::users::list_users,
            commands::users::create_user,
            commands::users::update_user,
            commands::users::change_user_pin,
            commands::users::delete_user,
            commands::printing::get_restaurant_info,
            commands::printing::save_restaurant_info,
            commands::printing::get_receipt_folder,
            commands::printing::set_receipt_folder,
            commands::printing::print_receipt,
            commands::printing::print_kitchen_ticket,
            commands::printing::open_file,
            commands::backups::list_backups,
            commands::backups::create_backup,
            commands::backups::restore_backup,
            commands::backups::delete_backup,
            commands::backups::get_backup_folder,
            commands::backups::set_backup_folder,
            commands::backups::get_auto_backup,
            commands::backups::set_auto_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}