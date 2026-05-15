use crate::vocab;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BuiltinBookDef {
    pub index: usize,
    pub title: String,
    pub filename: String,
}

#[tauri::command]
pub fn get_builtin_book_list() -> Vec<BuiltinBookDef> {
    vocab::builtin::BUILTIN_BOOKS.iter().enumerate().map(|(i, b)| BuiltinBookDef {
        index: i, title: b.title.to_string(), filename: b.filename.to_string(),
    }).collect()
}