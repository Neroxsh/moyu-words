use serde::{Deserialize, Serialize};

mod books_fn;
mod words_fn;
mod plans_fn;
mod units_fn;
mod unit_words_fn;
mod settings_fn;
mod builtin_fn;
mod overlay_fn;

// Re-export all command functions
pub use books_fn::*;
pub use words_fn::*;
pub use plans_fn::*;
pub use units_fn::*;
pub use unit_words_fn::*;
pub use settings_fn::*;
pub use builtin_fn::*;
pub use overlay_fn::*;

// Shared helper functions
pub(crate) fn iso_now() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub(crate) fn ceil_div(a: i64, b: i64) -> i64 {
    -(-a / b)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookSummary {
    pub id: i64,
    pub title: String,
    pub source_type: String,
    pub total_words: i64,
    pub active_words: i64,
    pub archived_words: i64,
    pub reviewed_words: i64,
    pub progress_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookRow {
    pub id: i64,
    pub title: String,
    pub source_type: String,
    pub source_ref: Option<String>,
    pub source_url: Option<String>,
    pub total_words: i64,
    pub created_at: String,
    pub updated_at: String,
    pub active: i64,
    pub actual_words: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordRow {
    pub id: i64,
    pub book_id: i64,
    pub seq: i64,
    pub word: String,
    pub meaning: String,
    pub raw: String,
    pub status: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanRow {
    pub id: i64,
    pub book_id: i64,
    pub days: i64,
    pub words_per_day: i64,
    pub created_at: String,
    pub active: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitRow {
    pub id: i64,
    pub plan_id: i64,
    pub unit_no: i64,
    pub start_seq: i64,
    pub end_seq: i64,
    pub status: String,
    pub completed_at: Option<String>,
    pub total_words: i64,
    pub pending_words: i64,
    pub reviewed_words: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitWordRow {
    pub id: i64,
    pub unit_id: i64,
    pub word_id: i64,
    pub position: i64,
    pub review_state: String,
    pub studied_at: Option<String>,
    pub word: String,
    pub meaning: String,
    pub word_status: String,
    pub seq: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentWord {
    pub unit_word_id: i64,
    pub word_id: i64,
    pub word: String,
    pub meaning: String,
    pub position: i64,
    pub total_words: i64,
    pub reviewed_words: i64,
    pub auto_running: bool,
}