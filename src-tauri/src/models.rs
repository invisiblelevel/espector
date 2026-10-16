//! Структуры данных, которые летят между Rust-бэкендом и Svelte-фронтендом
//! через Tauri IPC.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub root_path: String,
    pub files: Vec<IniFile>,
    pub plugins: Vec<PluginFile>,
    pub conflicts: Vec<ConflictGroup>,
    pub total_keys: usize,
    pub conflict_count: usize,
}

// ─────────────────────────────────────────────────────────────
// INI
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniFile {
    pub path: String,
    pub relative_path: String,
    pub name: String,
    pub size: u64,
    pub sections: Vec<IniSection>,
    pub conflicting_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniSection {
    pub name: String,
    pub keys: Vec<IniKey>,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniKey {
    pub name: String,
    pub value: String,
    pub line: usize,
    pub inline_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictGroup {
    pub full_key: String,
    pub section: String,
    pub key: String,
    pub occurrences: Vec<ConflictOccurrence>,
    pub values_differ: bool,
    pub unique_values_count: usize,
    pub winner: ConflictWinner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictOccurrence {
    pub file_relative_path: String,
    pub file_path: String,
    pub value: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictWinner {
    pub file_relative_path: String,
    pub file_path: String,
    pub value: String,
}

// ─────────────────────────────────────────────────────────────
// Плагины
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginFile {
    pub path: String,
    pub relative_path: String,
    pub name: String,
    pub size: u64,
    /// "esp", "esm" или "esl".
    pub kind: String,
    pub header: PluginHeader,
    pub total_records: usize,
    pub record_counts: Vec<RecordTypeCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginHeader {
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: f32,
    pub masters: Vec<String>,
    pub is_esm: bool,
    pub is_esl: bool,
    pub is_localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordTypeCount {
    /// Тип записи: "WEAP", "ARMO", "NPC_", "CELL", "QUST" и т.д.
    pub record_type: String,
    /// Сколько записей этого типа.
    pub count: usize,
}