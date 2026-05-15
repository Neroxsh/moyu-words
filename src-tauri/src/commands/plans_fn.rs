use crate::AppState;
use crate::commands::{PlanRow, WordRow, iso_now, ceil_div};
use tauri::State;

#[tauri::command]
pub fn get_active_plan(state: State<'_, AppState>, book_id: i64) -> Result<Option<PlanRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT * FROM plans WHERE book_id=?1 AND active=1 ORDER BY created_at DESC, id DESC LIMIT 1"
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map(rusqlite::params![book_id], |row| Ok(PlanRow {
        id: row.get(0)?, book_id: row.get(1)?, days: row.get(2)?,
        words_per_day: row.get(3)?, created_at: row.get(4)?, active: row.get(5)?,
    })).map_err(|e| e.to_string())?;
    let result: Option<PlanRow> = match rows.next() {
        Some(r) => Some(r.map_err(|e| e.to_string())?),
        None => None,
    };
    Ok(result)
}

#[tauri::command]
pub fn create_plan(state: State<'_, AppState>, book_id: i64, days: i64) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    // Get active words
    let mut stmt = db.conn.prepare(
        "SELECT * FROM words WHERE book_id=?1 AND status='active' ORDER BY seq ASC"
    ).map_err(|e| e.to_string())?;
    let words: Vec<WordRow> = stmt.query_map(rusqlite::params![book_id], |row| Ok(WordRow {
        id: row.get(0)?, book_id: row.get(1)?, seq: row.get(2)?,
        word: row.get(3)?, meaning: row.get(4)?, raw: row.get(5)?,
        status: row.get(6)?, archived_at: row.get(7)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;

    if words.is_empty() { return Err("这个词书还没有可学习的单词".to_string()); }

    let days = std::cmp::max(1, days);
    let words_per_day = std::cmp::max(1, ceil_div(words.len() as i64, days));
    let created_at = iso_now();

    db.conn.execute("UPDATE plans SET active=0 WHERE book_id=?1", rusqlite::params![book_id]).map_err(|e| e.to_string())?;
    db.conn.execute(
        "INSERT INTO plans(book_id, days, words_per_day, created_at, active) VALUES(?1,?2,?3,?4,1)",
        rusqlite::params![book_id, days, words_per_day, created_at],
    ).map_err(|e| e.to_string())?;
    let plan_id = db.conn.last_insert_rowid();

    let mut pos: usize = 0;
    for unit_no in 1..=days {
        let end = std::cmp::min(pos + words_per_day as usize, words.len());
        let slice = &words[pos..end];
        let start_seq = slice.first().map(|w| w.seq).unwrap_or((pos+1) as i64);
        let end_seq = slice.last().map(|w| w.seq).unwrap_or(pos as i64);

        db.conn.execute(
            "INSERT INTO units(plan_id, unit_no, start_seq, end_seq, status) VALUES(?1,?2,?3,?4,'pending')",
            rusqlite::params![plan_id, unit_no, start_seq, end_seq],
        ).map_err(|e| e.to_string())?;
        let unit_id = db.conn.last_insert_rowid();

        let mut uw_stmt = db.conn.prepare(
            "INSERT INTO unit_words(unit_id, word_id, position, review_state) VALUES(?1,?2,?3,'pending')"
        ).map_err(|e| e.to_string())?;
        for (idx, w) in slice.iter().enumerate() {
            uw_stmt.execute(rusqlite::params![unit_id, w.id, (idx+1) as i64]).map_err(|e| e.to_string())?;
        }
        pos = end;
    }

    // Set active book
    db.conn.execute("UPDATE books SET active=0", []).map_err(|e| e.to_string())?;
    db.conn.execute("UPDATE books SET active=1, updated_at=?1 WHERE id=?2", rusqlite::params![created_at, book_id]).map_err(|e| e.to_string())?;
    db.conn.execute("INSERT INTO settings(key,value) VALUES('active_plan_id',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value", rusqlite::params![plan_id.to_string()]).map_err(|e| e.to_string())?;
    Ok(plan_id)
}