use crate::AppState;
use crate::commands::WordRow;
use tauri::State;

#[tauri::command]
pub fn list_words(state: State<'_, AppState>, book_id: i64, include_archived: bool) -> Result<Vec<WordRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let query = if include_archived {
        "SELECT * FROM words WHERE book_id=?1 ORDER BY seq ASC"
    } else {
        "SELECT * FROM words WHERE book_id=?1 AND status='active' ORDER BY seq ASC"
    };
    let mut stmt = db.conn.prepare(query).map_err(|e| e.to_string())?;
    let rows: Vec<WordRow> = stmt.query_map(rusqlite::params![book_id], |row| Ok(WordRow {
        id: row.get(0)?, book_id: row.get(1)?, seq: row.get(2)?,
        word: row.get(3)?, meaning: row.get(4)?, raw: row.get(5)?,
        status: row.get(6)?, archived_at: row.get(7)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn get_archived_words(state: State<'_, AppState>, book_id: i64) -> Result<Vec<WordRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT * FROM words WHERE book_id=?1 AND status='archived' ORDER BY archived_at DESC, seq ASC"
    ).map_err(|e| e.to_string())?;
    let rows: Vec<WordRow> = stmt.query_map(rusqlite::params![book_id], |row| Ok(WordRow {
        id: row.get(0)?, book_id: row.get(1)?, seq: row.get(2)?,
        word: row.get(3)?, meaning: row.get(4)?, raw: row.get(5)?,
        status: row.get(6)?, archived_at: row.get(7)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn archive_word(state: State<'_, AppState>, word_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let now = crate::commands::iso_now();
    db.conn.execute("UPDATE words SET status='archived', archived_at=?1 WHERE id=?2", rusqlite::params![now, word_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn restore_word(state: State<'_, AppState>, word_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn.execute("UPDATE words SET status='active', archived_at=NULL WHERE id=?1", rusqlite::params![word_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn export_archived_words_csv(state: State<'_, AppState>, book_id: i64) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT * FROM words WHERE book_id=?1 AND status='archived' ORDER BY archived_at DESC, seq ASC"
    ).map_err(|e| e.to_string())?;
    let words: Vec<WordRow> = stmt.query_map(rusqlite::params![book_id], |row| Ok(WordRow {
        id: row.get(0)?, book_id: row.get(1)?, seq: row.get(2)?,
        word: row.get(3)?, meaning: row.get(4)?, raw: row.get(5)?,
        status: row.get(6)?, archived_at: row.get(7)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    let mut csv = String::from("word,meaning,seq,archived_at\n");
    for w in &words {
        csv.push_str(&format!("\"{}\",\"{}\",{},{}\n",
            w.word.replace('"', "\"\""), w.meaning.replace('"', "\"\""), w.seq,
            w.archived_at.as_deref().unwrap_or("")));
    }
    Ok(csv)
}