use crate::AppState;
use tauri::State;

#[tauri::command]
pub fn get_setting(state: State<'_, AppState>, key: String, default: String) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let result: Option<String> = db.conn.query_row(
        "SELECT value FROM settings WHERE key=?1", rusqlite::params![key], |row| row.get(0)
    ).ok();
    Ok(result.unwrap_or(default))
}

#[tauri::command]
pub fn set_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn.execute(
        "INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        rusqlite::params![key, value],
    ).map_err(|e| e.to_string())?;
    Ok(())
}