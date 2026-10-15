import type { Dict } from "./en";

export const ru: Dict = {
  toolbar: {
    pickFolder: "Выбрать папку",
    scanning: "Сканирую…",
    folder: "Папка",
    files: "Файлов",
    keys: "Ключей",
    conflicts: "Конфликтов",
    exportReport: "Экспорт отчёта",
    mergeIni: "Merged.ini",
    language: "Язык",
    info: "О программе",
  },
  tabs: {
    files: "Файлы",
    conflicts: "Конфликты",
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
  },
  viewer: {
    conflictTooltip: "Также в {count} файл(ах):",
  },
  emptyState: {
    pickFolderTitle: "Выберите папку для сканирования",
    pickFolderDescription:
      "ESPector найдёт все .ini файлы, разберёт их и покажет конфликты между модами.",
    pickFileTitle: "Выберите файл слева",
    pickFileDescription:
      "Кликните на любой .ini в списке, чтобы увидеть его содержимое.",
  },
  about: {
    close: "Закрыть",
    version: "Версия {v}",
    description:
      "Лёгкий инструмент для моддеров Bethesda. Сканирует .ini файлы, находит конфликты между модами, определяет победителя и генерирует merged.ini. Работает полностью офлайн — без облаков, подписок и сбора данных.",
    support: "Поддержать проект",
    license: "Лицензия MIT",
    copyright: "© 2026 ESPector",
  },
  support: {
    title: "Поддержать проект",
    subtitle: "Кофе разработчику",
    copy: "Копировать",
    thanks: "Спасибо за поддержку ❤️",
  },
  errors: {
    prefix: "Ошибка:",
  },
};