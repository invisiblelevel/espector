export const en = {
  toolbar: {
    pickFolder: "Choose Folder",
    scanning: "Scanning…",
    folder: "Folder",
    files: "Files",
    keys: "Keys",
    conflicts: "Conflicts",
    exportReport: "Export Report",
    mergeIni: "Merged.ini",
    language: "Language",
    info: "About",
  },
  tabs: {
    files: "Files",
    conflicts: "Conflicts",
  },
  fileList: {
    searchPlaceholder: "Search by file name…",
    onlyConflicts: "Only conflicted",
    filesCount: "Files",
    of: "of",
    nothingFound: "Nothing found",
  },
  conflictList: {
    title: "Conflicts",
    differingCount: "different values",
    sameCount: "identical values",
    noConflicts: "No conflicts found",
    valuesDiffer: "values differ",
    valuesSame: "values match",
    winner: "Winner",
    page: "line",
    uniqueValues: "unique values",
  },
  viewer: {
    conflictTooltip: "Also in {count} file(s):",
  },
  emptyState: {
    pickFolderTitle: "Choose a folder to scan",
    pickFolderDescription:
      "ESPector will find all .ini files, parse them, and show conflicts between mods.",
    pickFileTitle: "Choose a file on the left",
    pickFileDescription:
      "Click any .ini in the list to see its content.",
  },
  about: {
    close: "Close",
    version: "Version {v}",
    description:
      "A lightweight tool for Bethesda modders. Scans .ini files, detects conflicts between mods, identifies the winning value, and generates a merged.ini. Works fully offline — no clouds, no subscriptions, no data collection.",
    support: "Support the project",
    license: "MIT License",
    copyright: "© 2026 ESPector",
  },
  support: {
    title: "Support the project",
    subtitle: "A coffee for the dev",
    copy: "Copy",
    thanks: "Thanks for your support ❤️",
  },
  errors: {
    prefix: "Error:",
  },
};

export type Dict = typeof en;