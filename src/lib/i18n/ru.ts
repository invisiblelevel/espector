import type { Dict } from "./en";

export const ru: Dict = {
  toolbar: {
    pickFolder: "Выбрать папку",
    scanning: "Сканирую…",
    folder: "Папка",
    files: "Файлов",
    plugins: "Плагинов",
    keys: "Ключей",
    conflicts: "Конфликтов",
    exportIniReport: "Экспорт отчёта по INI",
    exportPluginsReport: "Экспорт отчёта по плагинам",
    mergeIni: "Merged.ini",
    language: "Язык",
    info: "О программе",
  },
  tabs: {
    files: "Файлы",
    conflicts: "Конфликты",
    plugins: "Плагины",
  },
  fileList: {
    searchPlaceholder: "Поиск по имени файла…",
    onlyConflicts: "Только конфликтные",
    filesCount: "Файлы",
    of: "из",
    nothingFound: "Ничего не найдено",
  },
  conflictList: {
    title: "Конфликты",
    differingCount: "с разными значениями",
    sameCount: "с одинаковыми",
    noConflicts: "Конфликтов не найдено",
    valuesDiffer: "значения разные",
    valuesSame: "значения совпадают",
    winner: "Победитель",
    page: "стр.",
    uniqueValues: "уникальных значений",
    copyValue: "Копировать значение",
  },
  plugins: {
    listTitle: "Плагины",
    recordsShort: "зап.",
    author: "Автор",
    version: "Версия",
    size: "Размер",
    records: "Записей",
    recordTypes: "Типы записей",
    description: "Описание",
    masters: "Мастер-файлы",
    noMasters: "Нет мастеров — независимый плагин",
    emptyTitle: "Плагинов не найдено",
    emptyDescription: "В сканированной папке нет .esp, .esm или .esl файлов.",
  },
  viewer: {
    conflictTooltip: "Также в {count} файл(ах):",
  },
  emptyState: {
    pickFolderTitle: "Выберите папку для сканирования",
    pickFolderDescription:
      "ESPector найдёт все .ini и файлы плагинов, разберёт их и покажет конфликты между модами.",
    pickFileTitle: "Выберите файл слева",
    pickFileDescription:
      "Кликните на любой файл в списке, чтобы увидеть его содержимое.",
  },
  about: {
    close: "Закрыть",
    version: "Версия {v}",
    description:
      "Инструмент для моддеров Bethesda. Сканирует .ini и файлы плагинов, находит конфликты между модами, определяет победителя, генерирует merged.ini и показывает, что внутри .esp/.esm/.esl.",
    support: "Поддержать проект",
    license: "Лицензия MIT",
    copyright: "© 2026 INV.LVL",
  },
  support: {
    title: "Поддержать проект",
    subtitle: "Разрабу на еду",
    copy: "Копировать",
    thanks: "Спасибо за поддержку ❤️",
  },
  errors: {
    prefix: "Ошибка:",
  },
};