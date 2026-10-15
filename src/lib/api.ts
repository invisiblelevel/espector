// TS-типы, зеркалящие Rust-модели из src-tauri/src/models.rs.

import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

/// Результат сканирования папки: файлы + сводка + конфликты.
export interface ScanResult {
  rootPath: string;
  files: IniFile[];
  conflicts: ConflictGroup[];
  totalKeys: number;
  conflictCount: number;
}

/// Один INI-файл.
export interface IniFile {
  path: string;
  relativePath: string;
  name: string;
  size: number;
  sections: IniSection[];
  conflictingKeys: string[];
}

/// Одна секция INI.
export interface IniSection {
  name: string;
  keys: IniKey[];
  comments: string[];
}

/// Один ключ INI.
export interface IniKey {
  name: string;
  value: string;
  line: number;
  inlineComment: string | null;
}

/// Один конфликтный ключ и все файлы, где он встречается.
export interface ConflictGroup {
  fullKey: string;
  section: string;
  key: string;
  occurrences: ConflictOccurrence[];
  valuesDiffer: boolean;
  uniqueValuesCount: number;
  winner: ConflictWinner;
}

/// Вхождение ключа в конкретном файле.
export interface ConflictOccurrence {
  fileRelativePath: string;
  filePath: string;
  value: string;
  line: number;
}

/// Файл-победитель конфликта.
export interface ConflictWinner {
  fileRelativePath: string;
  filePath: string;
  value: string;
}

// ─────────────────────────────────────────────────────────────
// Обёртки над Tauri-командами
// ─────────────────────────────────────────────────────────────

/// Сканирует указанную папку на .ini-файлы.
export async function scanFolder(path: string): Promise<ScanResult> {
  return invoke<ScanResult>("scan_folder", { path });
}

/// Открывает системный диалог выбора папки.
export async function pickFolder(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose folder to scan",
  });
  if (typeof selected === "string") return selected;
  return null;
}

/// Открывает системный диалог «Сохранить как» для отчёта.
export async function pickSavePath(defaultName: string): Promise<string | null> {
  const selected = await save({
    defaultPath: defaultName,
    filters: [
      { name: "Markdown", extensions: ["md"] },
      { name: "CSV", extensions: ["csv"] },
      { name: "All files", extensions: ["*"] },
    ],
  });
  return selected ?? null;
}

/// Открывает диалог «Сохранить как» специально для merged.ini.
/// Отдельная функция — потому что фильтры другие.
export async function pickSaveIniPath(defaultName: string): Promise<string | null> {
  const selected = await save({
    defaultPath: defaultName,
    filters: [
      { name: "INI", extensions: ["ini"] },
      { name: "All files", extensions: ["*"] },
    ],
  });
  return selected ?? null;
}

/// Сохраняет содержимое отчёта в файл по указанному пути.
export async function saveReport(path: string, content: string): Promise<void> {
  return invoke("save_report", { path, content });
}

/// Собирает merged.ini из результата сканирования.
/// Возвращает готовый текст — фронт сохраняет его через saveReport.
export async function buildMergedIni(result: ScanResult): Promise<string> {
  return invoke<string>("build_merged_ini", { result });
}