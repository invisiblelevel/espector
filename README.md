# ESPector

**Inspect mods. Resolve conflicts. Merge INI.** · **Инспектор модов. Конфликты. Merge INI.** · **模组检查。冲突解决。INI 合并。**

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)
[![Languages](https://img.shields.io/badge/languages-EN%20%7C%20RU%20%7C%20ZH-orange.svg)](#languages)

[English](#english) · [Русский](#русский) · [中文](#中文)

---

## English

ESPector is a lightweight desktop tool for Bethesda modders (Skyrim SE, Fallout 4). It scans a folder with loose `.ini` files, finds conflicts between mods, identifies the winning value, generates a `merged.ini`, and shows what's inside `.esp` / `.esm` / `.esl` plugins.

### Features

- **INI conflict scanner** — finds every `Section.Key` that appears in multiple files
- **Winner detection** — shows which file wins (by alphabetical load order, same as the game)
- **Merged.ini generator** — combines all keys into one file with source comments
- **Plugin inspector** — reads `.esp` / `.esm` / `.esl`: masters, flags (ESM/ESL), author, version, record types breakdown (WEAP, ARMO, NPC_, CELL, REFR, and more)
- **Two export formats** — Markdown (for forums) and CSV (for Excel)
- **Three languages** — English, Russian, Simplified Chinese, with system auto-detection
- **Dark theme**, fast and native (Rust + Tauri, no Electron bloat)

### How it works

1. Pick a folder with mods (loose `.ini` files and/or plugins).
2. ESPector recursively scans it, parses everything.
3. Open the **Files** tab to view any `.ini`, **Conflicts** tab to see conflicts, **Plugins** tab to inspect plugins.
4. Export a report (Markdown or CSV) or generate a `merged.ini`.

### Tech Stack

Tauri 2 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · Tailwind CSS 4 · esplugin

### Install

Download **ESPector Setup v1.0.0.exe** from [Releases](../../releases), run, install. No dependencies needed — WebView2 runtime is bundled.

Windows 10 version 1903 or newer / Windows 11 (64-bit).

### Build from source

Requirements:

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 with WebView2

Steps:

1. `git clone https://github.com/invisiblelevel/espector.git`
2. `cd espector`
3. `npm install`
4. `npm run tauri dev` — dev mode
5. `npm run tauri build` — release build

The installer will appear in `src-tauri/target/release/bundle/`.

### License

MIT — see [LICENSE](LICENSE).

### Author

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## Русский

ESPector — лёгкий десктопный инструмент для моддеров Bethesda (Skyrim SE, Fallout 4). Сканирует папку с loose `.ini` файлами, находит конфликты между модами, определяет победителя, генерирует `merged.ini` и показывает, что внутри плагинов `.esp` / `.esm` / `.esl`.

### Что умеет

- **Сканер INI-конфликтов** — находит каждый `Section.Key`, который встречается в нескольких файлах
- **Определение победителя** — показывает, чей файл побеждает (по алфавитному порядку загрузки, как в игре)
- **Генератор merged.ini** — собирает все ключи в один файл с комментариями-источниками
- **Инспектор плагинов** — читает `.esp` / `.esm` / `.esl`: мастер-листы, флаги (ESM/ESL), автор, версия, разбивка по типам записей (WEAP, ARMO, NPC_, CELL, REFR и другие)
- **Два формата экспорта** — Markdown (для форумов) и CSV (для Excel)
- **Три языка** — английский, русский, китайский (упрощённый), с авто-определением
- **Тёмная тема**, быстрый и нативный (Rust + Tauri, без раздутого Electron)

### Как работает

1. Выбери папку с модами (loose `.ini` и/или плагины).
2. ESPector рекурсивно сканирует её, парсит всё.
3. Открой вкладку **Файлы** для просмотра `.ini`, **Конфликты** для конфликтов, **Плагины** для разбора плагинов.
4. Экспортируй отчёт (Markdown или CSV) или сгенерируй `merged.ini`.

### Стек

Tauri 2 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · Tailwind CSS 4 · esplugin

### Установка

Скачай **ESPector Setup v1.0.0.exe** из раздела [Releases](../../releases), запусти, установи. Зависимости не нужны — WebView2 вшит в инсталлер.

Поддерживается Windows 10 (версия 1903+) / Windows 11 (64-bit).

### Сборка из исходников

Требования:

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 с WebView2

Шаги:

1. `git clone https://github.com/invisiblelevel/espector.git`
2. `cd espector`
3. `npm install`
4. `npm run tauri dev` — dev-режим
5. `npm run tauri build` — релиз

Готовый установщик появится в `src-tauri/target/release/bundle/`.

### Лицензия

MIT — см. [LICENSE](LICENSE).

### Автор

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## 中文

ESPector 是一款面向 Bethesda 模组制作者的轻量桌面工具（Skyrim SE、Fallout 4）。它扫描包含松散 `.ini` 文件的文件夹，找出模组之间的冲突，确定胜出值，生成 `merged.ini`，并显示 `.esp` / `.esm` / `.esl` 插件的内部内容。

### 功能

- **INI 冲突扫描器** — 查找出现在多个文件中的每个 `Section.Key`
- **胜出者检测** — 显示哪个文件胜出（按字母顺序加载，与游戏一致）
- **merged.ini 生成器** — 将所有键合并到一个文件中，并附带来源注释
- **插件检查器** — 读取 `.esp` / `.esm` / `.esl`：主文件、标志（ESM/ESL）、作者、版本、记录类型分布（WEAP、ARMO、NPC_、CELL、REFR 等）
- **两种导出格式** — Markdown（用于论坛）和 CSV（用于 Excel）
- **三种语言** — 英语、俄语、简体中文，支持系统自动检测
- **深色主题**，快速且原生（Rust + Tauri，无臃肿的 Electron）

### 使用方法

1. 选择包含模组的文件夹（松散 `.ini` 文件和/或插件）。
2. ESPector 递归扫描并解析所有内容。
3. 打开 **文件** 标签页查看 `.ini`，**冲突** 标签页查看冲突，**插件** 标签页检查插件。
4. 导出报告（Markdown 或 CSV）或生成 `merged.ini`。

### 技术栈

Tauri 2 · SvelteKit + Svelte 5 (runes) · TypeScript · Rust · Tailwind CSS 4 · esplugin

### 安装

从 [Releases](../../releases) 下载 **ESPector Setup v1.0.0.exe**，运行并安装。无需任何依赖 — 安装包内置 WebView2。

支持 Windows 10（1903 或更新版本）/ Windows 11（64 位）。

### 从源码构建

环境要求：

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- Windows 10/11 且已安装 WebView2

步骤：

1. `git clone https://github.com/invisiblelevel/espector.git`
2. `cd espector`
3. `npm install`
4. `npm run tauri dev` — 开发模式
5. `npm run tauri build` — 发布构建

安装包将生成于 `src-tauri/target/release/bundle/`。

### 许可证

MIT — 见 [LICENSE](LICENSE)。

### 作者

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)