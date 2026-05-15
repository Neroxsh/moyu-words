use crate::AppState;
use crate::commands::{BookRow, BookSummary, iso_now};
use crate::vocab;
use tauri::{Manager, State};

#[tauri::command]
pub fn list_books(state: State<'_, AppState>) -> Result<Vec<BookRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT b.*, COALESCE(COUNT(w.id), 0) AS actual_words
         FROM books b LEFT JOIN words w ON w.book_id = b.id
         GROUP BY b.id ORDER BY b.active DESC, b.updated_at DESC, b.id DESC",
    ).map_err(|e| e.to_string())?;
    let rows: Vec<BookRow> = stmt.query_map([], |row| Ok(BookRow {
        id: row.get(0)?, title: row.get(1)?, source_type: row.get(2)?,
        source_ref: row.get(3)?, source_url: row.get(4)?, total_words: row.get(5)?,
        created_at: row.get(6)?, updated_at: row.get(7)?, active: row.get(8)?, actual_words: row.get(9)?,
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;
    Ok(rows)
}

#[tauri::command]
pub fn get_book(state: State<'_, AppState>, book_id: i64) -> Result<Option<BookRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare(
        "SELECT b.*, COALESCE(COUNT(w.id), 0) AS actual_words
         FROM books b LEFT JOIN words w ON w.book_id = b.id WHERE b.id = ?1 GROUP BY b.id",
    ).map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map(rusqlite::params![book_id], |row| Ok(BookRow {
        id: row.get(0)?, title: row.get(1)?, source_type: row.get(2)?,
        source_ref: row.get(3)?, source_url: row.get(4)?, total_words: row.get(5)?,
        created_at: row.get(6)?, updated_at: row.get(7)?, active: row.get(8)?, actual_words: row.get(9)?,
    })).map_err(|e| e.to_string())?;
    let result: Option<BookRow> = match rows.next() {
        Some(r) => Some(r.map_err(|e| e.to_string())?),
        None => None,
    };
    Ok(result)
}

#[tauri::command]
pub fn get_active_book(state: State<'_, AppState>) -> Result<Option<BookRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    // Try active book first
    let result: Option<BookRow> = {
        let mut stmt = db.conn.prepare(
            "SELECT b.*, COALESCE(COUNT(w.id), 0) AS actual_words
             FROM books b LEFT JOIN words w ON w.book_id = b.id WHERE b.active = 1
             GROUP BY b.id ORDER BY b.updated_at DESC, b.id DESC LIMIT 1",
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query_map([], |row| Ok(BookRow {
            id: row.get(0)?, title: row.get(1)?, source_type: row.get(2)?,
            source_ref: row.get(3)?, source_url: row.get(4)?, total_words: row.get(5)?,
            created_at: row.get(6)?, updated_at: row.get(7)?, active: row.get(8)?, actual_words: row.get(9)?,
        })).map_err(|e| e.to_string())?;
        match rows.next() {
            Some(r) => Some(r.map_err(|e| e.to_string())?),
            None => None,
        }
    };
    if result.is_some() { return Ok(result); }
    // Fallback to most recent book
    let mut stmt2 = db.conn.prepare(
        "SELECT b.*, COALESCE(COUNT(w.id), 0) AS actual_words
         FROM books b LEFT JOIN words w ON w.book_id = b.id
         GROUP BY b.id ORDER BY b.updated_at DESC, b.id DESC LIMIT 1",
    ).map_err(|e| e.to_string())?;
    let mut rows2 = stmt2.query_map([], |row| Ok(BookRow {
        id: row.get(0)?, title: row.get(1)?, source_type: row.get(2)?,
        source_ref: row.get(3)?, source_url: row.get(4)?, total_words: row.get(5)?,
        created_at: row.get(6)?, updated_at: row.get(7)?, active: row.get(8)?, actual_words: row.get(9)?,
    })).map_err(|e| e.to_string())?;
    match rows2.next() {
        Some(r) => r.map(|v| Some(v)).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn set_active_book(state: State<'_, AppState>, book_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let now = iso_now();
    db.conn.execute("UPDATE books SET active = 0", []).map_err(|e| e.to_string())?;
    db.conn.execute("UPDATE books SET active = 1, updated_at = ?1 WHERE id = ?2", rusqlite::params![now, book_id]).map_err(|e| e.to_string())?;
    db.conn.execute("INSERT INTO settings(key, value) VALUES('active_book_id', ?1) ON CONFLICT(key) DO UPDATE SET value = excluded.value", rusqlite::params![book_id.to_string()]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn add_book_from_file(state: State<'_, AppState>, title: String, file_path: String) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let path = std::path::Path::new(&file_path);
    let payload = std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown.txt");
    let entries = vocab::parser::parse_book_payload(filename, &payload)?;
    if entries.is_empty() { return Err("没有解析到任何单词".to_string()); }
    let created_at = iso_now();
    db.conn.execute(
        "INSERT INTO books(title, source_type, source_ref, source_url, total_words, created_at, updated_at, active) VALUES(?1,?2,?3,?4,?5,?6,?6,0)",
        rusqlite::params![title, "local", file_path, "", entries.len() as i64, created_at],
    ).map_err(|e| e.to_string())?;
    let book_id = db.conn.last_insert_rowid();
    let mut stmt = db.conn.prepare("INSERT INTO words(book_id, seq, word, meaning, raw, status) VALUES(?1,?2,?3,?4,?5,'active')").map_err(|e| e.to_string())?;
    for (seq, e) in entries.iter().enumerate() {
        stmt.execute(rusqlite::params![book_id, (seq+1) as i64, e.word, e.meaning, e.raw]).map_err(|e| e.to_string())?;
    }
    Ok(book_id)
}

#[tauri::command]
pub fn add_builtin_book(state: State<'_, AppState>, app: tauri::AppHandle, book_index: usize) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let book = vocab::builtin::BUILTIN_BOOKS.get(book_index).ok_or("无效的内置词书索引")?;
    let source_ref = format!("builtin:{}", book.filename);
    let existing: Option<i64> = db.conn.query_row("SELECT id FROM books WHERE source_type='builtin' AND source_ref=?1 LIMIT 1", rusqlite::params![source_ref], |row| row.get(0)).ok();
    if let Some(id) = existing {
        let now = iso_now();
        db.conn.execute("UPDATE books SET active=0", []).map_err(|e| e.to_string())?;
        db.conn.execute("UPDATE books SET active=1, updated_at=?1 WHERE id=?2", rusqlite::params![now, id]).map_err(|e| e.to_string())?;
        db.conn.execute("INSERT INTO settings(key,value) VALUES('active_book_id',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value", rusqlite::params![id.to_string()]).map_err(|e| e.to_string())?;
        return Ok(id);
    }
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?.join("resources").join("vocab");
    let cache_dir = app.path().app_data_dir().map_err(|e| e.to_string())?.join("cache");
    std::fs::create_dir_all(&cache_dir).ok();
    let local_path = resource_dir.join(book.filename);
    let payload = if local_path.exists() && local_path.metadata().map(|m| m.len()).unwrap_or(0) > 0 {
        std::fs::read_to_string(&local_path).map_err(|e| e.to_string())?
    } else {
        let cache_path = cache_dir.join(format!("builtin_{}", book.filename));
        if cache_path.exists() && cache_path.metadata().map(|m| m.len()).unwrap_or(0) > 0 {
            std::fs::read_to_string(&cache_path).map_err(|e| e.to_string())?
        } else {
            let url = vocab::builtin::builtin_book_url(book);
            let resp = ureq::get(&url).set("User-Agent", "Mozilla/5.0").call().map_err(|e| format!("下载失败: {}", e))?;
            let text = resp.into_string().map_err(|e| format!("读取失败: {}", e))?;
            std::fs::write(&cache_path, &text).map_err(|e| e.to_string())?;
            text
        }
    };
    let entries = vocab::parser::parse_book_payload(book.filename, &payload)?;
    if entries.is_empty() { return Err("没有解析到任何单词".to_string()); }
    let created_at = iso_now();
    let url = vocab::builtin::builtin_book_url(book);
    db.conn.execute(
        "INSERT INTO books(title, source_type, source_ref, source_url, total_words, created_at, updated_at, active) VALUES(?1,'builtin',?2,?3,?4,?5,?5,0)",
        rusqlite::params![book.title, source_ref, url, entries.len() as i64, created_at],
    ).map_err(|e| e.to_string())?;
    let book_id = db.conn.last_insert_rowid();
    let mut stmt = db.conn.prepare("INSERT INTO words(book_id, seq, word, meaning, raw, status) VALUES(?1,?2,?3,?4,?5,'active')").map_err(|e| e.to_string())?;
    for (seq, e) in entries.iter().enumerate() {
        stmt.execute(rusqlite::params![book_id, (seq+1) as i64, e.word, e.meaning, e.raw]).map_err(|e| e.to_string())?;
    }
    let now = iso_now();
    db.conn.execute("UPDATE books SET active=0", []).map_err(|e| e.to_string())?;
    db.conn.execute("UPDATE books SET active=1, updated_at=?1 WHERE id=?2", rusqlite::params![now, book_id]).map_err(|e| e.to_string())?;
    db.conn.execute("INSERT INTO settings(key,value) VALUES('active_book_id',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value", rusqlite::params![book_id.to_string()]).map_err(|e| e.to_string())?;
    Ok(book_id)
}

#[tauri::command]
pub fn delete_book(state: State<'_, AppState>, book_id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn.execute("DELETE FROM books WHERE id=?1", rusqlite::params![book_id]).map_err(|e| e.to_string())?;
    let fallback: Option<i64> = db.conn.query_row("SELECT id FROM books ORDER BY updated_at DESC, id DESC LIMIT 1", [], |row| row.get(0)).ok();
    db.conn.execute("UPDATE books SET active=0", []).map_err(|e| e.to_string())?;
    if let Some(fid) = fallback {
        let now = iso_now();
        db.conn.execute("UPDATE books SET active=1, updated_at=?1 WHERE id=?2", rusqlite::params![now, fid]).map_err(|e| e.to_string())?;
        db.conn.execute("INSERT INTO settings(key,value) VALUES('active_book_id',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value", rusqlite::params![fid.to_string()]).map_err(|e| e.to_string())?;
    } else {
        db.conn.execute("DELETE FROM settings WHERE key IN ('active_book_id','active_plan_id')", []).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_book_summary(state: State<'_, AppState>, book_id: i64) -> Result<BookSummary, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let total: i64 = db.conn.query_row("SELECT COUNT(*) FROM words WHERE book_id=?1", rusqlite::params![book_id], |row| row.get(0)).map_err(|e| e.to_string())?;
    let archived: i64 = db.conn.query_row("SELECT COUNT(*) FROM words WHERE book_id=?1 AND status='archived'", rusqlite::params![book_id], |row| row.get(0)).map_err(|e| e.to_string())?;
    let reviewed: i64 = db.conn.query_row(
        "SELECT COUNT(DISTINCT w.id) FROM words w LEFT JOIN unit_words uw ON uw.word_id=w.id WHERE w.book_id=?1 AND (w.status='archived' OR uw.review_state IN ('known','unknown'))",
        rusqlite::params![book_id], |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    let active = total - archived;
    let book_title: String = db.conn.query_row("SELECT title FROM books WHERE id=?1", rusqlite::params![book_id], |row| row.get(0)).unwrap_or_else(|_| "未命名词书".into());
    let source_type: String = db.conn.query_row("SELECT source_type FROM books WHERE id=?1", rusqlite::params![book_id], |row| row.get(0)).unwrap_or_else(|_| "local".into());
    let total_max = if total == 0 { 1 } else { total };
    Ok(BookSummary {
        id: book_id, title: book_title, source_type,
        total_words: total, active_words: active, archived_words: archived, reviewed_words: reviewed,
        progress_text: format!("{}/{} ({}%)", reviewed, total, (reviewed*100/total_max)),
    })
}