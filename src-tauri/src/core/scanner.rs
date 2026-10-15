//! Сканер папки: рекурсивно находит все .ini, парсит их через `core::ini`,
//! собирает сводку и группирует конфликты между файлами.

use crate::core::ini;
use crate::models::{ConflictGroup, ConflictOccurrence, ConflictWinner, ScanResult};
use anyhow::{Context, Result};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

/// Сканирует папку `root`, находит все `.ini` рекурсивно, парсит их
/// и возвращает `ScanResult` со сводкой и конфликтами.
pub fn scan_folder(root: &Path) -> Result<ScanResult> {
    if !root.exists() {
        anyhow::bail!("Папка не существует: {}", root.display());
    }
    if !root.is_dir() {
        anyhow::bail!("Это не папка: {}", root.display());
    }

    // 1. Собираем пути ко всем .ini.
    let ini_paths: Vec<_> = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("ini"))
                .unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect();

    // 2. Парсим каждый файл и сразу проставляем relative_path.
    let mut files = Vec::with_capacity(ini_paths.len());
    for path in &ini_paths {
        match ini::parse_file(path) {
            Ok(mut f) => {
                f.relative_path = compute_relative(root, path);
                files.push(f);
            }
            Err(e) => {
                eprintln!("[scanner] пропущен {}: {e:#}", path.display());
            }
        }
    }

    // 3. Строим карту: "Section.Key" -> [(file_idx, value, line)].
    let mut usage: BTreeMap<String, Vec<OccurrenceRaw>> = BTreeMap::new();
    for (file_idx, file) in files.iter().enumerate() {
        let mut seen_in_file: HashSet<String> = HashSet::new();
        for section in &file.sections {
            for key in &section.keys {
                let full = format!("{}.{}", section.name, key.name);
                if seen_in_file.insert(full.clone()) {
                    usage.entry(full).or_default().push(OccurrenceRaw {
                        file_idx,
                        value: key.value.clone(),
                        line: key.line,
                    });
                }
            }
        }
    }

    // 4. Отбираем конфликты и собираем ConflictGroup.
    let mut conflicts: Vec<ConflictGroup> = Vec::new();
    let mut conflicting_set: HashSet<String> = HashSet::new();

    for (full_key, occurrences) in &usage {
        let unique_files: HashSet<usize> = occurrences.iter().map(|o| o.file_idx).collect();
        if unique_files.len() < 2 {
            continue;
        }

        conflicting_set.insert(full_key.clone());
        let (section, key_name) = split_full_key(full_key);

        // Сортируем вхождения по relative_path.
        let mut occs_sorted = occurrences.clone();
        occs_sorted.sort_by(|a, b| {
            files[a.file_idx]
                .relative_path
                .cmp(&files[b.file_idx].relative_path)
        });

        let mapped: Vec<ConflictOccurrence> = occs_sorted
            .iter()
            .map(|o| ConflictOccurrence {
                file_relative_path: files[o.file_idx].relative_path.clone(),
                file_path: files[o.file_idx].path.clone(),
                value: o.value.clone(),
                line: o.line,
            })
            .collect();

        // Уникальные значения.
        let unique_values: HashSet<&str> = mapped.iter().map(|o| o.value.as_str()).collect();
        let unique_values_count = unique_values.len();

        // Победитель — файл с максимальным relative_path (алфавитный порядок).
        // В Bethesda loose .ini из Data/ читаются в алфавитном порядке,
        // поэтому последний по алфавиту перезаписывает все предыдущие.
        let winner_occ = occs_sorted
            .last()
            .expect("occurrences is non-empty by construction");
        let winner = ConflictWinner {
            file_relative_path: files[winner_occ.file_idx].relative_path.clone(),
            file_path: files[winner_occ.file_idx].path.clone(),
            value: winner_occ.value.clone(),
        };

        conflicts.push(ConflictGroup {
            full_key: full_key.clone(),
            section,
            key: key_name,
            occurrences: mapped,
            values_differ: unique_values_count > 1,
            unique_values_count,
            winner,
        });
    }

    // 5. Проставляем флаг конфликта в каждый файл.
    for file in files.iter_mut() {
        for section in &file.sections {
            for key in &section.keys {
                let full = format!("{}.{}", section.name, key.name);
                if conflicting_set.contains(&full) {
                    file.conflicting_keys.push(full);
                }
            }
        }
    }

    let total_keys = files
        .iter()
        .map(|f| f.sections.iter().map(|s| s.keys.len()).sum::<usize>())
        .sum();

    Ok(ScanResult {
        root_path: root.to_string_lossy().to_string(),
        files,
        conflicts,
        total_keys,
        conflict_count: conflicting_set.len(),
    })
}

/// Обёртка для Tauri.
pub fn scan_folder_str(path: &str) -> Result<ScanResult> {
    let root = Path::new(path);
    scan_folder(root).with_context(|| format!("Скан не удался для: {path}"))
}

// ─────────────────────────────────────────────────────────────
// Внутренние типы и помощники
// ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OccurrenceRaw {
    file_idx: usize,
    value: String,
    line: usize,
}

/// Вычисляет путь файла относительно корня скана.
fn compute_relative(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative
        .to_string_lossy()
        .to_string()
        .replace('\\', "/")
}

/// Разбивает "Section.Key" на (section, key).
fn split_full_key(full: &str) -> (String, String) {
    match full.find('.') {
        Some(pos) => (full[..pos].to_string(), full[pos + 1..].to_string()),
        None => (String::new(), full.to_string()),
    }
}