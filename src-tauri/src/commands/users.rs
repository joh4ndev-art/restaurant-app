use tauri::State;

use crate::domain::{
    ChangePinInput, CreateUserInput, LoginInput, UpdateUserInput, User,
};
use crate::error::AppResult;
use crate::services::UserService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn has_users(state: State<'_, AppState>) -> AppResult<bool> {
    let conn = state.conn()?;
    UserService::has_users(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn login(state: State<'_, AppState>, input: LoginInput) -> AppResult<User> {
    let conn = state.conn()?;
    UserService::login(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn list_users(state: State<'_, AppState>) -> AppResult<Vec<User>> {
    let conn = state.conn()?;
    UserService::list(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_user(
    state: State<'_, AppState>,
    input: CreateUserInput,
) -> AppResult<User> {
    let conn = state.conn()?;
    UserService::create(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_user(
    state: State<'_, AppState>,
    input: UpdateUserInput,
) -> AppResult<User> {
    let conn = state.conn()?;
    UserService::update(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn change_user_pin(
    state: State<'_, AppState>,
    input: ChangePinInput,
) -> AppResult<()> {
    let conn = state.conn()?;
    UserService::change_pin(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_user(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.conn()?;
    UserService::delete(&conn, id)
}