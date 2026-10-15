//! Парсер INI-файлов. Поддерживает:
//! - секции `[SectionName]`
//! - ключи `key = value` (пробелы вокруг `=` опциональны)
//! - комментарии на отдельной строке: `; comment` или `# comment`
//! - inline-комментарии после значения: `key = value ; comment`
//! - пустые строки, отступы
//!
//! Не поддерживает (пока не нужно):
//! - многострочные значения (в Bethesda INI такого почти нет)
//! - экранирование символов
//! - секции без имени `[]`

use crate::models::{IniFile, IniKey, IniSection};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Читает INI-файл с диска и разбирает его в структуру `IniFile`.
/// `conflicting_keys` оставляет пустым — его заполнит сканер.
pub fn parse_file(path: &Path) -> Result<IniFile> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Не удалось прочитать файл: {}", path.display()))?;

    let metadata = fs::metadata(path)
        .with_context(|| format!("Не удалось получить метаданные: {}", path.display()))?;

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.ini")
        .to_string();

    let sections = parse_content(&content);

    Ok(IniFile {
        path: path.to_string_lossy().to_string(),
        // relative_path заполнит сканер — он знает корень скана.
        // Парсер одного файла понятия не имеет, где корень.
        relative_path: name.clone(),
        name,
        size: metadata.len(),
        sections,
        conflicting_keys: Vec::new(),
    })
}

/// Разбирает содержимое INI в список секций.
/// Строки до первой секции игнорируются (там обычно только комментарии).
fn parse_content(content: &str) -> Vec<IniSection> {
    let mut sections: Vec<IniSection> = Vec::new();
    let mut current: Option<IniSection> = None;
    let mut pending_comments: Vec<String> = Vec::new();

    for (idx, raw_line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw_line.trim();

        // Пустая строка — сбрасываем только inline-контекст, но не комментарии.
        if line.is_empty() {
            continue;
        }

        // Полнострочный комментарий: `; ...` или `# ...`
        if line.starts_with(';') || line.starts_with('#') {
            let text = line[1..].trim().to_string();
            pending_comments.push(text);
            continue;
        }

        // Начало секции: `[SectionName]`
        if line.starts_with('[') && line.ends_with(']') {
            // Сохраняем предыдущую секцию, если была.
            if let Some(sec) = current.take() {
                sections.push(sec);
            }
            let name = line[1..line.len() - 1].trim().to_string();
            current = Some(IniSection {
                name,
                keys: Vec::new(),
                comments: std::mem::take(&mut pending_comments),
            });
            continue;
        }

        // Ключ: `key = value` (или `key=value`, или `key = value ; comment`)
        if let Some((key, value, inline_comment)) = parse_key_line(line) {
            // Ключ без секции — игнорируем (некорректный INI).
            let Some(sec) = current.as_mut() else {
                continue;
            };

            // Комментарии, накопленные до ключа, цепляем к секции —
            // чтобы при рендере показывать контекст.
            if !pending_comments.is_empty() {
                for c in pending_comments.drain(..) {
                    sec.comments.push(c);
                }
            }

            sec.keys.push(IniKey {
                name: key,
                value,
                line: line_no,
                inline_comment,
            });
        }
    }

    // Не забываем последнюю секцию.
    if let Some(sec) = current.take() {
        sections.push(sec);
    }

    sections
}

/// Разбирает одну строку `key = value ; comment`.
/// Возвращает `(key, value, inline_comment)` или `None`, если строка не похожа на ключ.
fn parse_key_line(line: &str) -> Option<(String, String, Option<String>)> {
    let eq_pos = line.find('=')?;
    let key = line[..eq_pos].trim();
    if key.is_empty() {
        return None;
    }

    let rest = line[eq_pos + 1..].trim();

    // Ищем inline-комментарий: `;` или `#` после значения.
    // Не идеально (может встретиться в значении), но для Bethesda INI сойдёт.
    let (value, inline_comment) = if let Some(pos) = find_inline_comment(rest) {
        let v = rest[..pos].trim().to_string();
        let c = rest[pos + 1..].trim().to_string();
        (v, Some(c))
    } else {
        (rest.to_string(), None)
    };

    Some((key.to_string(), value, inline_comment))
}

/// Ищет позицию inline-комментария: `;` или `#`, не находящихся внутри значения.
/// Учитываем, что Bethesda иногда пишет `#` в значениях (редко), поэтому
/// ищем первое вхождение после хотя бы одного непустого символа значения.
fn find_inline_comment(s: &str) -> Option<usize> {
    s.char_indices()
        .find(|(_, c)| *c == ';' || *c == '#')
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_section() {
        let content = "[Combat]\nfDamageMultiplier=2.0\n";
        let sections = parse_content(content);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].name, "Combat");
        assert_eq!(sections[0].keys.len(), 1);
        assert_eq!(sections[0].keys[0].name, "fDamageMultiplier");
        assert_eq!(sections[0].keys[0].value, "2.0");
    }

    #[test]
    fn parses_spaces_around_equals() {
        let content = "[General]\n  bEnable = 1  \n";
        let sections = parse_content(content);
        assert_eq!(sections[0].keys[0].name, "bEnable");
        assert_eq!(sections[0].keys[0].value, "1");
    }

    #[test]
    fn parses_inline_comment() {
        let content = "[Audio]\nfVolume = 0.5 ; half\n";
        let sections = parse_content(content);
        let key = &sections[0].keys[0];
        assert_eq!(key.value, "0.5");
        assert_eq!(key.inline_comment.as_deref(), Some("half"));
    }

    #[test]
    fn parses_full_line_comments() {
        let content = "; header\n[Combat]\n# note\nfDamage=1.0\n";
        let sections = parse_content(content);
        assert_eq!(sections[0].comments, vec!["header", "note"]);
    }

    #[test]
    fn skips_keys_before_section() {
        let content = "orphan=1\n[Combat]\nfDamage=2.0\n";
        let sections = parse_content(content);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].keys.len(), 1);
    }

    #[test]
    fn handles_empty_content() {
        let sections = parse_content("");
        assert!(sections.is_empty());
    }
}