//! Структуры данных, которые летят между Rust-бэкендом и Svelte-фронтендом
//! через Tauri IPC. Все поля сериализуются в camelCase — так фронту удобнее.

use serde::{Deserialize, Serialize};

/// Результат сканирования папки: список найденных INI-файлов,
/// сводка и сгруппированные конфликты.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    /// Абсолютный путь к папке, которую просканировали.
    pub root_path: String,
    /// Список всех найденных .ini файлов.
    pub files: Vec<IniFile>,
    /// Сгруппированные конфликты.
    pub conflicts: Vec<ConflictGroup>,
    /// Сколько всего ключей во всех файлах.
    pub total_keys: usize,
    /// Сколько уникальных конфликтных ключей.
    pub conflict_count: usize,
}

/// Один INI-файл: путь, имя, разобранные секции и список конфликтных ключей.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniFile {
    /// Абсолютный путь к файлу.
    pub path: String,
    /// Путь относительно корня скана.
    pub relative_path: String,
    /// Имя файла без пути.
    pub name: String,
    /// Размер файла в байтах.
    pub size: u64,
    /// Секции в порядке появления в файле.
    pub sections: Vec<IniSection>,
    /// Ключи этого файла, которые пересекаются с другими файлами.
    pub conflicting_keys: Vec<String>,
}

/// Одна секция INI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniSection {
    /// Имя секции без квадратных скобок.
    pub name: String,
    /// Ключи в порядке появления.
    pub keys: Vec<IniKey>,
    /// Комментарии, которые шли до этой секции.
    pub comments: Vec<String>,
}

/// Один ключ: `key = value` и его метаданные.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IniKey {
    /// Имя ключа (без пробелов).
    pub name: String,
    /// Значение как строка.
    pub value: String,
    /// Номер строки в файле (1-based).
    pub line: usize,
    /// Комментарий на той же строке после значения.
    pub inline_comment: Option<String>,
}

/// Один конфликтный ключ и все места, где он встречается.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictGroup {
    /// Полное имя ключа в формате "Section.Key".
    pub full_key: String,
    /// Имя секции.
    pub section: String,
    /// Имя ключа.
    pub key: String,
    /// Все файлы, где встречается ключ, с их значениями.
    /// Отсортировано по relative_path для стабильного порядка.
    pub occurrences: Vec<ConflictOccurrence>,
    /// true, если значения различаются.
    pub values_differ: bool,
    /// Сколько уникальных значений в группе.
    pub unique_values_count: usize,
    /// Файл-победитель (тот, чьё значение реально применяется
    /// при загрузке Data/*.ini в алфавитном порядке).
    /// Если файлов несколько с одинаковым максимальным путём — берём последний.
    pub winner: ConflictWinner,
}

/// Одно вхождение конфликтного ключа в конкретном файле.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictOccurrence {
    /// Путь к файлу относительно корня скана.
    pub file_relative_path: String,
    /// Абсолютный путь к файлу.
    pub file_path: String,
    /// Значение ключа в этом файле.
    pub value: String,
    /// Номер строки.
    pub line: usize,
}

/// Победитель конфликта: файл, чьё значение применяется.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictWinner {
    /// Путь к файлу-победителю относительно корня.
    pub file_relative_path: String,
    /// Абсолютный путь к файлу-победителю.
    pub file_path: String,
    /// Значение, которое применяется.
    pub value: String,
}