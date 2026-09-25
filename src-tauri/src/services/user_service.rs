use rand::Rng;
use rusqlite::Connection;
use sha2::{Digest, Sha256};

use crate::domain::{
    ChangePinInput, CreateUserInput, LoginInput, Role, UpdateUserInput, User,
};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::UserRepository;

pub struct UserService;

impl UserService {
    /// True if at least one user exists. Used on startup to decide
    /// whether to show the login screen.
    pub fn has_users(conn: &Connection) -> AppResult<bool> {
        Ok(UserRepository::new(conn).count()? > 0)
    }

    pub fn list(conn: &Connection) -> AppResult<Vec<User>> {
        UserRepository::new(conn).list()
    }

    /// Verify credentials and return the matching user.
    pub fn login(conn: &Connection, input: LoginInput) -> AppResult<User> {
        let username = input.username.trim().to_string();
        if username.is_empty() || input.pin.is_empty() {
            return Err(AppError::Validation(
                "username and PIN are required".into(),
            ));
        }

        let repo = UserRepository::new(conn);
        let creds = repo
            .get_credentials_by_username(&username)?
            .ok_or_else(|| AppError::Validation("invalid credentials".into()))?;

        if !creds.active {
            return Err(AppError::Validation("user is inactive".into()));
        }

        let computed = hash_pin(&input.pin, &creds.pin_salt);
        if computed != creds.pin_hash {
            return Err(AppError::Validation("invalid credentials".into()));
        }

        repo.get_by_id(creds.id)
    }

    pub fn create(conn: &Connection, input: CreateUserInput) -> AppResult<User> {
        let username = input.username.trim().to_string();
        validate_username(&username)?;
        validate_pin(&input.pin)?;

        let repo = UserRepository::new(conn);
        if repo.exists_by_username(&username, None)? {
            return Err(AppError::Validation(format!(
                "a user named '{username}' already exists"
            )));
        }

        let salt = generate_salt();
        let hash = hash_pin(&input.pin, &salt);

        repo.create(&username, &hash, &salt, input.role)
    }

    pub fn update(conn: &Connection, input: UpdateUserInput) -> AppResult<User> {
        let username = input.username.trim().to_string();
        validate_username(&username)?;

        let repo = UserRepository::new(conn);
        let existing = repo.get_by_id(input.id)?;

        if repo.exists_by_username(&username, Some(input.id))? {
            return Err(AppError::Validation(format!(
                "a user named '{username}' already exists"
            )));
        }

        // Refuse to demote or deactivate the last active admin.
        let was_active_admin =
            existing.role == Role::Admin && existing.active;
        let will_stay_active_admin =
            input.role == Role::Admin && input.active;

        if was_active_admin && !will_stay_active_admin {
            let admins = repo.count_admins()?;
            if admins <= 1 {
                return Err(AppError::Validation(
                    "cannot demote or deactivate the last active admin".into(),
                ));
            }
        }

        repo.update(input.id, &username, input.role, input.active)
    }

    pub fn change_pin(conn: &Connection, input: ChangePinInput) -> AppResult<()> {
        validate_pin(&input.pin)?;

        let repo = UserRepository::new(conn);
        repo.get_by_id(input.id)?; // ensure it exists

        let salt = generate_salt();
        let hash = hash_pin(&input.pin, &salt);

        repo.update_pin(input.id, &hash, &salt)
    }

    pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
        let repo = UserRepository::new(conn);
        let existing = repo.get_by_id(id)?;

        // Refuse to delete the last active admin.
        if existing.role == Role::Admin && existing.active {
            let admins = repo.count_admins()?;
            if admins <= 1 {
                return Err(AppError::Validation(
                    "cannot delete the last active admin".into(),
                ));
            }
        }

        repo.delete(id)
    }
}

// ---------- helpers ----------

fn hash_pin(pin: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(pin.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn generate_salt() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn validate_username(username: &str) -> AppResult<()> {
    if username.is_empty() {
        return Err(AppError::Validation("username cannot be empty".into()));
    }
    if username.chars().count() > 40 {
        return Err(AppError::Validation(
            "username cannot exceed 40 characters".into(),
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(AppError::Validation(
            "username may only contain letters, digits, '_', '-' and '.'".into(),
        ));
    }
    Ok(())
}

fn validate_pin(pin: &str) -> AppResult<()> {
    if pin.len() < 4 || pin.len() > 8 {
        return Err(AppError::Validation(
            "PIN must be between 4 and 8 digits".into(),
        ));
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::Validation("PIN must contain only digits".into()));
    }
    Ok(())
}