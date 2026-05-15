use crate::AppState;
use crate::commands::{UnitRow, iso_now};
use tauri::State;

#[tauri::command]
pub fn list_units(state: State<'_, AppState>, plan_id: i64) -> Result<Vec<UnitRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT u.*, COUNT(uw.id) AS total_words,
         SUM(CASE WHEN uw.review_state='pending' THEN 1 ELSE 0 END) AS pending_words,
         SUM(CASE WHEN uw.review_state IN ('known','unknown') THEN 1 ELSE 0 END) AS reviewed_words
         FROM units u LEFT JOIN unit_words uw ON uw.unit_id=u.id
         WHERE u.plan_id=?1 GROUP BY u.id ORDER BY u.unit_no ASC"
    ).map_err(|e| e.to_string())?;
    let rows: Vec<UnitRow> = stmt.query_map(rusqlite::params![plan_id], |row| Ok(UnitRow {
        id: row.get(0)?, plan_id: row.get(1)?, unit_no: row.get(2)?,
        start_seq: row.get(3)?, end_seq: row.get(4)?, status: row.get(5)?,
        completed_at: row.get(6)?, total_words: row.get(7)?,
        pending_words: row.get(8)?, reviewed_words: row.get(9)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn get_unit_info(state: State<'_, AppState>, unit_id: i64) -> Result<Option<UnitRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT u.*, COUNT(uw.id) AS total_words,
         SUM(CASE WHEN uw.review_state='pending' THEN 1 ELSE 0 END) AS pending_words,
         SUM(CASE WHEN uw.review_state IN ('known','unknown') THEN 1 ELSE 0 END) AS reviewed_words
         FROM units u LEFT JOIN unit_words uw ON uw.unit_id=u.id
         WHERE u.id=?1 GROUP BY u.id"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map(rusqlite::params![unit_id], |row| Ok(UnitRow {
        id: row.get(0)?, plan_id: row.get(1)?, unit_no: row.get(2)?,
        start_seq: row.get(3)?, end_seq: row.get(4)?, status: row.get(5)?,
        completed_at: row.get(6)?, total_words: row.get(7)?,
        pending_words: row.get(8)?, reviewed_words: row.get(9)?,
    })).map_err(|e| e.to_string())?;
    let result: Option<UnitRow> = match rows.next() {
        Some(r) => Some(r.map_err(|e| e.to_string())?),
        None => None,
    };
    Ok(result)
}

#[tauri::command]
pub fn mark_unit_doing(state: State<'_, AppState>, unit_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn.execute("UPDATE units SET status='doing' WHERE id=?1", rusqlite::params![unit_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn mark_unit_done(state: State<'_, AppState>, unit_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let now = iso_now();
    db.conn.execute("UPDATE units SET status='done', completed_at=?1 WHERE id=?2", rusqlite::params![now, unit_id]).map_err(|e| e.to_string())?;
    Ok(())
}