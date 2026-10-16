//! Сканер папки: рекурсивно находит .ini и плагины Bethesda,
//! парсит их и собирает сводку + конфликты INI.

use crate::core::{esp, ini};
use crate::models::{ConflictGroup, ConflictOccurrence, ConflictWinner, ScanResult};
use anyhow::{Context, Result};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_folder(root: &Path) -> Result<ScanResult> {
    if !root.exists() {
        anyhow::bail!("Папка не существует: {}", root.display());
    }
    if !root.is_dir() {
        anyhow::bail!("Это не папка: {}", root.display());
    }

    // 1. Собираем все файлы рекурсивно.
    let all_paths: Vec<_> = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    let ini_paths: Vec<_> = all_paths
        .iter()
        .filter(|p| {
            p.extension()
                .map(|ext| ext.eq_ignore_ascii_case("ini"))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let plugin_paths: Vec<_> = all_paths
        .iter()
        .filter(|p| {
            p.extension()
                .map(|ext| {
                    let e = ext.to_string_lossy().to_lowercase();
                    e == "esp" || e == "esm" || e == "esl"
                })
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    // 2. Парсим INI.
    let mut files = Vec::with_capacity(ini_paths.len());
    for path in &ini_paths {
        match ini::parse_file(path) {
            Ok(mut f) => {
                f.relative_path = compute_relative(root, path);
                files.push(f);
            }
            Err(e) => {
                eprintln!("[scanner] пропущен INI {}: {e:#}", path.display());
            }
        }
    }

    // 3. Парсим плагины.
    let mut plugins = Vec::with_capacity(plugin_paths.len());
    for path in &plugin_paths {
        match esp::parse_plugin(path, root) {
            Ok(p) => plugins.push(p),
            Err(e) => {
                eprintln!("[scanner] пропущен плагин {}: {e:#}", path.display());
            }
        }
    }

    // 4. Конфликты INI.
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

    let mut conflicts: Vec<ConflictGroup> = Vec::new();
    let mut conflicting_set: HashSet<String> = HashSet::new();

    for (full_key, occurrences) in &usage {
        let unique_files: HashSet<usize> = occurrences.iter().map(|o| o.file_idx).collect();
        if unique_files.len() < 2 {
            continue;
        }

        conflicting_set.insert(full_key.clone());
        let (section, key_name) = split_full_key(full_key);

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

        let unique_values: HashSet<&str> = mapped.iter().map(|o| o.value.as_str()).collect();
        let unique_values_count = unique_values.len();

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
        plugins,
        conflicts,
        total_keys,
        conflict_count: conflicting_set.len(),
    })
}

pub fn scan_folder_str(path: &str) -> Result<ScanResult> {
    let root = Path::new(path);
    scan_folder(root).with_context(|| format!("Скан не удался для: {path}"))
}

// ─────────────────────────────────────────────────────────────
// Помощники
// ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OccurrenceRaw {
    file_idx: usize,
    value: String,
    line: usize,
}

fn compute_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
        .replace('\\', "/")
}

fn split_full_key(full: &str) -> (String, String) {
    match full.find('.') {
        Some(pos) => (full[..pos].to_string(), full[pos + 1..].to_string()),
        None => (String::new(), full.to_string()),
    }
}