//! Ядро ESPector: парсинг INI, сканирование папок, генерация merged.ini.
//! Ничего не знает про Tauri — чистый Rust, тестируется отдельно.

pub mod ini;
pub mod merge;
pub mod scanner;