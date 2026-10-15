//! Tauri-команды — тонкий мост между фронтендом и ядром.
//! Никакой логики здесь нет: только валидация входных данных,
//! вызов ядра и преобразование ошибок в строки для IPC.

use crate::core::{merge, scanner};
use crate::models::ScanResult;
use std::fs;
use std::path::Path;

/// Сканирует указанную папку на предмет .ini-файлов.
#[tauri::command]
pub async fn scan_folder(path: String) -> Result<ScanResult, String> {
    if path.trim().is_empty() {
        return Err("Путь к папке не указан".to_string());
    }

    let result = tauri::async_runtime::spawn_blocking(move || {
        scanner::scan_folder_str(&path)
    })
    .await
    .map_err(|e| format!("Задача сканирования упала: {e}"))?;

    result.map_err(|e| format!("{e:#}"))
}

/// Сохраняет содержимое отчёта в файл по указанному пути.
#[tauri::command]
pub async fn save_report(path: String, content: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("Путь к файлу не указан".to_string());
    }

    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        let p = Path::new(&path);

        if let Some(parent) = p.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Не удалось создать папку {}: {e}", parent.display()))?;
            }
        }

        fs::write(p, content)
            .map_err(|e| format!("Не удалось записать файл {}: {e}", p.display()))?;

        Ok(())
    })
    .await
    .map_err(|e| format!("Задача сохранения упала: {e}"))?
}

/// Собирает merged.ini из результата сканирования.
/// Возвращает готовый текст файла — фронт сам сохранит его.
#[tauri::command]
pub async fn build_merged_ini(result: ScanResult) -> Result<String, String> {
    let text = tauri::async_runtime::spawn_blocking(move || {
        merge::build_merged_ini(&result)
    })
    .await
    .map_err(|e| format!("Задача генерации merged.ini упала: {e}"))?;

    Ok(text)
}