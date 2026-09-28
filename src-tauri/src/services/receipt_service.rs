use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::domain::{Order, PrintOutput, PrintResult, RestaurantInfo};
use crate::error::{AppError, AppResult};
use crate::infrastructure::printing;
use crate::infrastructure::repositories::{OrderRepository, SettingsRepository};

const KEY_NAME: &str    = "restaurant_name";
const KEY_ADDRESS: &str = "restaurant_address";
const KEY_PHONE: &str   = "restaurant_phone";
const KEY_FOOTER: &str  = "restaurant_footer";
const KEY_FOLDER: &str  = "receipt_folder";

pub struct ReceiptService;

impl ReceiptService {
    /// Read the restaurant info from settings.
    pub fn restaurant_info(conn: &Connection) -> AppResult<RestaurantInfo> {
        let repo = SettingsRepository::new(conn);
        Ok(RestaurantInfo {
            name: repo.get(KEY_NAME)?.unwrap_or_else(|| "Restaurant".into()),
            address: repo.get(KEY_ADDRESS)?,
            phone: repo.get(KEY_PHONE)?,
            footer_message: repo.get(KEY_FOOTER)?,
        })
    }

    /// Save restaurant info to settings.
    pub fn save_restaurant_info(
        conn: &Connection,
        info: &RestaurantInfo,
    ) -> AppResult<()> {
        let repo = SettingsRepository::new(conn);
        repo.set(KEY_NAME, info.name.trim())?;
        repo.set(KEY_ADDRESS, info.address.as_deref().unwrap_or(""))?;
        repo.set(KEY_PHONE, info.phone.as_deref().unwrap_or(""))?;
        repo.set(KEY_FOOTER, info.footer_message.as_deref().unwrap_or(""))?;
        Ok(())
    }

    /// Resolve the output folder, creating it if it doesn't exist.
    /// Falls back to the app data directory if the custom folder is unavailable.
    pub fn resolve_output_folder(
        app: &AppHandle,
        conn: &Connection,
    ) -> AppResult<PathBuf> {
        let repo = SettingsRepository::new(conn);
        let configured = repo.get(KEY_FOLDER)?.filter(|s| !s.trim().is_empty());

        let folder = match configured {
            Some(path) => PathBuf::from(path),
            None => {
                // Default: ~/Documents/restaurant-receipts
                let docs = app
                    .path()
                    .document_dir()
                    .map_err(|e| AppError::Internal(format!("no document dir: {e}")))?;
                docs.join("restaurant-receipts")
            }
        };

        fs::create_dir_all(&folder)
            .map_err(|e| AppError::Internal(format!("cannot create folder: {e}")))?;

        Ok(folder)
    }

    /// Save the configured folder path.
    pub fn set_output_folder(conn: &Connection, path: &str) -> AppResult<()> {
        SettingsRepository::new(conn).set(KEY_FOLDER, path.trim())
    }

    /// Generate a sales receipt PDF for the given order.
    pub fn generate_receipt(
        app: &AppHandle,
        conn: &Connection,
        order_id: i64,
    ) -> AppResult<PrintResult> {
        let order = OrderRepository::new(conn).get_order(order_id)?;

        // Only paid orders get a sales receipt.
        if order.status != crate::domain::OrderStatus::Paid {
            return Err(AppError::Validation(
                "receipt can only be generated for paid orders".into(),
            ));
        }

        let restaurant = Self::restaurant_info(conn)?;
        let folder = Self::resolve_output_folder(app, conn)?;
        let filename = format!("receipt-{}-{}.pdf", order.id, timestamp_slug(&order.created_at));
        let path = folder.join(filename);

        printing::pdf::generate_receipt(&order, &restaurant, &path)?;

        Ok(PrintResult {
            path: path.to_string_lossy().into_owned(),
            output: PrintOutput::Pdf,
        })
    }

    /// Generate a kitchen ticket PDF for the given order.
    pub fn generate_kitchen_ticket(
        app: &AppHandle,
        conn: &Connection,
        order_id: i64,
    ) -> AppResult<PrintResult> {
        let order = OrderRepository::new(conn).get_order(order_id)?;
        let folder = Self::resolve_output_folder(app, conn)?;
        let filename = format!("kitchen-{}-{}.pdf", order.id, timestamp_slug(&order.created_at));
        let path = folder.join(filename);

        printing::pdf::generate_kitchen_ticket(&order, &path)?;

        Ok(PrintResult {
            path: path.to_string_lossy().into_owned(),
            output: PrintOutput::Pdf,
        })
    }
}

/// Turn a timestamp into a filename-safe slug.
/// "2026-09-28 15:30:45" -> "20260928-153045"
fn timestamp_slug(ts: &str) -> String {
    ts.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .take(14)
        .collect()
}

/// Open a file with the OS default application (Linux: xdg-open).
pub fn open_path(path: &Path) -> AppResult<()> {
    let result = if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open").arg(path).spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(path).spawn()
    } else if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(path)
            .spawn()
    } else {
        return Err(AppError::Internal("unsupported OS".into()));
    };

    result.map_err(|e| AppError::Internal(format!("cannot open file: {e}")))?;
    Ok(())
}

/// Unused import guard for Order.
#[allow(dead_code)]
fn _type_check(_: &Order) {}