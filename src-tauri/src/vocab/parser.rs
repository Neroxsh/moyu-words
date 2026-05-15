use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordEntry {
    pub word: String,
    pub meaning: String,
    pub raw: String,
}

pub fn parse_book_payload(filename: &str, payload: &str) -> Result<Vec<WordEntry>, String> {
    let suffix = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt")
        .to_lowercase();

    match suffix.as_str() {
        "json" => parse_json_book(payload),
        "csv" => parse_csv_book(payload),
        _ => parse_text_book(payload),
    }
}

fn parse_text_book(payload: &str) -> Result<Vec<WordEntry>, String> {
    let mut rows = Vec::new();
    for line in payload.lines() {
        let raw = line.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (word, meaning) = if let Some(pos) = raw.find('\t') {
            (raw[..pos].trim().to_string(), raw[pos + 1..].trim().to_string())
        } else if let Some(pos) = raw.find("  ") {
            (raw[..pos].trim().to_string(), raw[pos + 2..].trim().to_string())
        } else if let Some(pos) = raw.find(' ') {
            (raw[..pos].trim().to_string(), raw[pos + 1..].trim().to_string())
        } else {
            (raw.to_string(), String::new())
        };

        if word.is_empty() {
            continue;
        }
        rows.push(WordEntry {
            word,
            meaning,
            raw: raw.to_string(),
        });
    }
    Ok(rows)
}

fn parse_json_book(payload: &str) -> Result<Vec<WordEntry>, String> {
    let data: serde_json::Value =
        serde_json::from_str(payload).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let items: Vec<&serde_json::Value> = if let Some(words) = data.get("words").and_then(|v| v.as_array()) {
        words.iter().collect()
    } else if data.is_object() {
        vec![&data]
    } else if let Some(arr) = data.as_array() {
        arr.iter().collect()
    } else {
        return Err("JSON 词书格式不正确".to_string());
    };

    let mut rows = Vec::new();
    for item in items {
        let obj = match item.as_object() {
            Some(o) => o,
            None => continue,
        };
        let word = obj
            .get("word")
            .or_else(|| obj.get("english"))
            .or_else(|| obj.get("en"))
            .or_else(|| obj.get("headword"))
            .or_else(|| obj.get("term"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();

        let meaning = obj
            .get("meaning")
            .or_else(|| obj.get("chinese"))
            .or_else(|| obj.get("zh"))
            .or_else(|| obj.get("translation"))
            .or_else(|| obj.get("definition"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();

        let raw = serde_json::to_string(item).unwrap_or_default();

        if !word.is_empty() {
            rows.push(WordEntry { word, meaning, raw });
        }
    }
    Ok(rows)
}

fn parse_csv_book(payload: &str) -> Result<Vec<WordEntry>, String> {
    let mut rows = Vec::new();
    for line in payload.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.is_empty() {
            continue;
        }
        let word = parts[0].trim().to_string();
        let meaning = if parts.len() > 1 {
            parts[1..].join(",").trim().to_string()
        } else {
            String::new()
        };
        if !word.is_empty() {
            rows.push(WordEntry {
                word,
                meaning,
                raw: trimmed.to_string(),
            });
        }
    }
    Ok(rows)
}