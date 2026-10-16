//! Парсер .esp/.esm/.esl плагинов Bethesda через `esplugin`.
//!
//! Извлекаем заголовок: автор, описание, версия, флаги, мастера.
//! Разбивку по типам записей считаем своим парсером `esp_records`.

use crate::models::{PluginFile, PluginHeader};
use anyhow::{Context, Result};
use esplugin::{GameId, ParseOptions, Plugin};
use std::path::Path;

/// Парсит один плагин и возвращает `PluginFile`.
pub fn parse_plugin(path: &Path, root: &Path) -> Result<PluginFile> {
    let game_id = detect_game_id(path);

    let mut plugin = Plugin::new(game_id, path);

    plugin
        .parse_file(ParseOptions::whole_plugin())
        .with_context(|| format!("Не удалось распарсить плагин: {}", path.display()))?;

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.esp")
        .to_string();

    let relative_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
        .replace('\\', "/");

    let metadata = std::fs::metadata(path)
        .with_context(|| format!("Не удалось получить метаданные: {}", path.display()))?;

    let is_esm = plugin.is_master_file();
    let is_esl = plugin.is_light_plugin();

    let kind = if is_esl {
        "esl".to_string()
    } else if is_esm {
        "esm".to_string()
    } else {
        "esp".to_string()
    };

    let masters = plugin.masters().unwrap_or_default();
    let description = plugin.description().ok().flatten();
    let version = plugin.header_version().unwrap_or(0.0);

    let author = description.as_deref().and_then(extract_author);

    // Разбивка по типам записей — своим парсером.
    let record_counts = crate::core::esp_records::count_record_types(path)
        .unwrap_or_default();
    let total_records: usize = record_counts.iter().map(|r| r.count).sum();

    Ok(PluginFile {
        path: path.to_string_lossy().to_string(),
        relative_path,
        name,
        size: metadata.len(),
        kind,
        header: PluginHeader {
            author,
            description,
            version,
            masters,
            is_esm,
            is_esl,
            is_localized: false,
        },
        total_records,
        record_counts,
    })
}

/// Определяет GameId. Для чтения заголовка подходит SkyrimSE — формат
/// заголовка и мастер-листов одинаков для SSE/FO4/Starfield.
fn detect_game_id(_path: &Path) -> GameId {
    GameId::SkyrimSE
}

/// Пытается вытащить автора из описания.
fn extract_author(description: &str) -> Option<String> {
    let first_line = description.lines().next()?.trim();

    let lower = first_line.to_lowercase();
    for prefix in ["author:", "author -", "author "] {
        if let Some(pos) = lower.find(prefix) {
            let rest = first_line[pos + prefix.len()..].trim();
            let author = rest.trim_start_matches([':', '-', ' ']).trim();
            if !author.is_empty() {
                return Some(author.to_string());
            }
        }
    }

    if let Some(pos) = lower.find("by ") {
        let rest = first_line[pos + 3..].trim();
        if !rest.is_empty() {
            return Some(rest.to_string());
        }
    }

    None
}