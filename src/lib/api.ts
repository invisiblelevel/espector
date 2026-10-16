// TS-типы, зеркалящие Rust-модели из src-tauri/src/models.rs.

import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

export interface ScanResult {
  rootPath: string;
  files: IniFile[];
  plugins: PluginFile[];
  conflicts: ConflictGroup[];
  totalKeys: number;
  conflictCount: number;
}

// ─── INI ───────────────────────────────────────────────────

export interface IniFile {
  path: string;
  relativePath: string;
  name: string;
  size: number;
  sections: IniSection[];
  conflictingKeys: string[];
}

export interface IniSection {
  name: string;
  keys: IniKey[];
  comments: string[];
}

export interface IniKey {
  name: string;
  value: string;
  line: number;
  inlineComment: string | null;
}

export interface ConflictGroup {
  fullKey: string;
  section: string;
  key: string;
  occurrences: ConflictOccurrence[];
  valuesDiffer: boolean;
  uniqueValuesCount: number;
  winner: ConflictWinner;
}

export interface ConflictOccurrence {
  fileRelativePath: string;
  filePath: string;
  value: string;
  line: number;
}

export interface ConflictWinner {
  fileRelativePath: string;
  filePath: string;
  value: string;
}

// ─── Плагины ───────────────────────────────────────────────

export interface PluginFile {
  path: string;
  relativePath: string;
  name: string;
  size: number;
  kind: string;
  header: PluginHeader;
  totalRecords: number;
  recordCounts: RecordTypeCount[];
}

export interface PluginHeader {
  author: string | null;
  description: string | null;
  version: number;
  masters: string[];
  isEsm: boolean;
  isEsl: boolean;
  isLocalized: boolean;
}

export interface RecordTypeCount {
  recordType: string;
  count: number;
}

// ─────────────────────────────────────────────────────────────
// Обёртки над Tauri-командами
// ─────────────────────────────────────────────────────────────

export async function scanFolder(path: string): Promise<ScanResult> {
  return invoke<ScanResult>("scan_folder", { path });
}

export async function pickFolder(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose folder to scan",
  });
  if (typeof selected === "string") return selected;
  return null;
}

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

export async function saveReport(path: string, content: string): Promise<void> {
  return invoke("save_report", { path, content });
}

export async function buildMergedIni(result: ScanResult): Promise<string> {
  return invoke<string>("build_merged_ini", { result });
}