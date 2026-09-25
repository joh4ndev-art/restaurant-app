use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::{Role, User};
use crate::error::{AppError, AppResult};

/// Internal row containing the stored hash and salt.
/// Never returned to the frontend.
pub struct UserCredentials {
    pub id: i64,
    pub username: String,
    pub pin_hash: String,
    pub pin_salt: String,
    pub role: Role,
    pub active: bool,
}

pub struct UserRepository<'a> {
    conn: &'a Connection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn count(&self) -> AppResult<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM users",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn list(&self) -> AppResult<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, role, active, created_at
             FROM users
             ORDER BY username ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut users = Vec::new();
        for row in rows {
            let r = row?;
            let role = Role::from_db(&r.2).map_err(AppError::Internal)?;
            users.push(User {
                id: r.0,
                username: r.1,
                role,
                active: r.3 != 0,
                created_at: r.4,
            });
        }
        Ok(users)
    }

    pub fn get_by_id(&self, id: i64) -> AppResult<User> {
        let user = self.conn.query_row(
            "SELECT id, username, role, active, created_at
             FROM users WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::NotFound(format!("user with id {id} not found"))
            }
            other => AppError::Database(other),
        })?;

        let role = Role::from_db(&user.2).map_err(AppError::Internal)?;
        Ok(User {
            id: user.0,
            username: user.1,
            role,
            active: user.3 != 0,
            created_at: user.4,
        })
    }

    pub fn get_credentials_by_username(
        &self,
        username: &str,
    ) -> AppResult<Option<UserCredentials>> {
        let row = self.conn.query_row(
            "SELECT id, username, pin_hash, pin_salt, role, active
             FROM users WHERE username = ?1",
            params![username],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, i32>(5)?,
                ))
            },
        ).optional()?;

        match row {
            None => Ok(None),
            Some(r) => {
                let role = Role::from_db(&r.4).map_err(AppError::Internal)?;
                Ok(Some(UserCredentials {
                    id: r.0,
                    username: r.1,
                    pin_hash: r.2,
                    pin_salt: r.3,
                    role,
                    active: r.5 != 0,
                }))
            }
        }
    }

    pub fn create(
        &self,
        username: &str,
        pin_hash: &str,
        pin_salt: &str,
        role: Role,
    ) -> AppResult<User> {
        self.conn.execute(
            "INSERT INTO users (username, pin_hash, pin_salt, role)
             VALUES (?1, ?2, ?3, ?4)",
            params![username, pin_hash, pin_salt, role.as_str()],
        )?;
        let id = self.conn.last_insert_rowid();
        self.get_by_id(id)
    }

    pub fn update(
        &self,
        id: i64,
        username: &str,
        role: Role,
        active: bool,
    ) -> AppResult<User> {
        let affected = self.conn.execute(
            "UPDATE users
             SET username   = ?1,
                 role       = ?2,
                 active     = ?3,
                 updated_at = datetime('now')
             WHERE id = ?4",
            params![username, role.as_str(), active as i32, id],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("user with id {id} not found")));
        }
        self.get_by_id(id)
    }

    pub fn update_pin(&self, id: i64, pin_hash: &str, pin_salt: &str) -> AppResult<()> {
        let affected = self.conn.execute(
            "UPDATE users
             SET pin_hash   = ?1,
                 pin_salt   = ?2,
                 updated_at = datetime('now')
             WHERE id = ?3",
            params![pin_hash, pin_salt, id],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("user with id {id} not found")));
        }
        Ok(())
    }

    pub fn delete(&self, id: i64) -> AppResult<()> {
        let affected = self
            .conn
            .execute("DELETE FROM users WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("user with id {id} not found")));
        }
        Ok(())
    }

    pub fn exists_by_username(&self, username: &str, exclude_id: Option<i64>) -> AppResult<bool> {
        let count: i64 = match exclude_id {
            Some(exclude) => self.conn.query_row(
                "SELECT COUNT(*) FROM users WHERE username = ?1 AND id != ?2",
                params![username, exclude],
                |row| row.get(0),
            )?,
            None => self.conn.query_row(
                "SELECT COUNT(*) FROM users WHERE username = ?1",
                params![username],
                |row| row.get(0),
            )?,
        };
        Ok(count > 0)
    }

    pub fn count_admins(&self) -> AppResult<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM users WHERE role = 'admin' AND active = 1",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}