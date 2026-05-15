use crate::commands::UnitWordRow;
use crate::db::Database;
use tauri::{AppHandle, Emitter, Listener, Manager, WebviewUrl, WebviewWindowBuilder};

#[derive(serde::Serialize, Clone)]
pub struct OverlayInitPayload {
    pub unit_id: i64,
}

#[tauri::command]
pub fn create_overlay_window(app: AppHandle, label: String, unit_id: i64) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window(&label) {
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        existing.emit("init-overlay", OverlayInitPayload { unit_id }).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(&app, &label, WebviewUrl::App("overlay.html".into()))
        .title("摸鱼窗口")
        .inner_size(400.0, 60.0)
        .min_inner_size(200.0, 36.0)
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .resizable(true)
        .visible(false)
        .shadow(false)
        .build()
        .map_err(|e| e.to_string())?;

    let label_clone = label.clone();
    window.once("ready", move |_| {
        if let Some(w) = app.get_webview_window(&label_clone) {
            let _ = w.show();
            let _ = w.emit("init-overlay", OverlayInitPayload { unit_id });
        }
    });

    Ok(())
}

#[tauri::command]
pub fn close_overlay_window(app: AppHandle, label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn hide_overlay_window(app: AppHandle, label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&label) {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Internal helper to query unit words (used by overlay commands)
fn list_unit_words_inner(db: &Database, unit_id: i64, pending_only: bool) -> Result<Vec<UnitWordRow>, String> {
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
pub fn get_current_word(
    state: tauri::State<'_, crate::AppState>,
    unit_id: i64, queue_index: usize,
) -> Result<Option<crate::commands::CurrentWord>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let words = list_unit_words_inner(&db, unit_id, true)?;
    let all_words = list_unit_words_inner(&db, unit_id, false)?;
    if words.is_empty() { return Ok(None); }
    let idx = queue_index.min(words.len() - 1);
    let row = &words[idx];
    let total = all_words.len() as i64;
    let reviewed = (total - words.len() as i64).max(0);
    Ok(Some(crate::commands::CurrentWord {
        unit_word_id: row.id, word_id: row.word_id,
        word: row.word.clone(), meaning: row.meaning.clone(),
        position: (idx + 1) as i64, total_words: total, reviewed_words: reviewed,
        auto_running: false,
    }))
}

#[tauri::command]
pub fn go_prev_word(queue_len: usize, current_idx: usize) -> usize {
    if queue_len == 0 { 0 } else { (current_idx + queue_len - 1) % queue_len }
}

#[tauri::command]
pub fn go_next_word(queue_len: usize, current_idx: usize) -> usize {
    if queue_len == 0 { 0 } else { (current_idx + 1) % queue_len }
}

#[tauri::command]
pub fn toggle_auto(current: bool) -> bool {
    !current
}