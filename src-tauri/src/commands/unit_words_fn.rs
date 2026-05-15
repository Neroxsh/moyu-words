use crate::AppState;
use crate::commands::{UnitWordRow, iso_now};
use tauri::State;

#[tauri::command]
pub fn list_unit_words(state: State<'_, AppState>, unit_id: i64, pending_only: bool) -> Result<Vec<UnitWordRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let query = if pending_only {
        "SELECT uw.*, w.word, w.meaning, w.status AS word_status, w.seq
         FROM unit_words uw JOIN words w ON w.id=uw.word_id
         WHERE uw.unit_id=?1 AND uw.review_state='pending' AND w.status='active'
         ORDER BY uw.position ASC"
    } else {
        "SELECT uw.*, w.word, w.meaning, w.status AS word_status, w.seq
         FROM unit_words uw JOIN words w ON w.id=uw.word_id
         WHERE uw.unit_id=?1 ORDER BY uw.position ASC"
    };
    let mut stmt = db.conn.prepare(query).map_err(|e| e.to_string())?;
    let rows: Vec<UnitWordRow> = stmt.query_map(rusqlite::params![unit_id], |row| Ok(UnitWordRow {
        id: row.get(0)?, unit_id: row.get(1)?, word_id: row.get(2)?,
        position: row.get(3)?, review_state: row.get(4)?, studied_at: row.get(5)?,
        word: row.get(6)?, meaning: row.get(7)?, word_status: row.get(8)?, seq: row.get(9)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn mark_unit_word(state: State<'_, AppState>, unit_word_id: i64, review_state: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let now = iso_now();
    db.conn.execute("UPDATE unit_words SET review_state=?1, studied_at=?2 WHERE id=?3", rusqlite::params![review_state, now, unit_word_id]).map_err(|e| e.to_string())?;
    Ok(())
}