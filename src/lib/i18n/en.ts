export const en = {
  toolbar: {
    pickFolder: "Choose Folder",
    scanning: "Scanning…",
    folder: "Folder",
    files: "Files",
    plugins: "Plugins",
    keys: "Keys",
    conflicts: "Conflicts",
    exportIniReport: "Export INI report",
    exportPluginsReport: "Export plugins report",
    mergeIni: "Merged.ini",
    language: "Language",
    info: "About",
  },
  tabs: {
    files: "Files",
    conflicts: "Conflicts",
    plugins: "Plugins",
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
    copyValue: "Copy value",
  },
  plugins: {
    listTitle: "Plugins",
    recordsShort: "rec.",
    author: "Author",
    version: "Version",
    size: "Size",
    records: "Records",
    recordTypes: "Record types",
    description: "Description",
    masters: "Masters",
    noMasters: "No masters — standalone plugin",
    emptyTitle: "No plugins found",
    emptyDescription: "No .esp, .esm or .esl files in the scanned folder.",
  },
  viewer: {
    conflictTooltip: "Also in {count} file(s):",
  },
  emptyState: {
    pickFolderTitle: "Choose a folder to scan",
    pickFolderDescription:
      "ESPector will find all .ini and plugin files, parse them, and show conflicts between mods.",
    pickFileTitle: "Choose a file on the left",
    pickFileDescription:
      "Click any file in the list to see its content.",
  },
  about: {
    close: "Close",
    version: "Version {v}",
    description:
      "A tool for Bethesda modders. Scans .ini and plugin files, detects conflicts between mods, identifies the winning value, generates a merged.ini, and shows what's inside .esp/.esm/.esl plugins.",
    support: "Support the project",
    license: "MIT License",
    copyright: "© 2026 INV.LVL",
  },
  support: {
    title: "Support the project",
    subtitle: "A food for the dev",
    copy: "Copy",
    thanks: "Thanks for your support ❤️",
  },
  errors: {
    prefix: "Error:",
  },
};

export type Dict = typeof en;