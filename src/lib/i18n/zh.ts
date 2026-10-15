import type { Dict } from "./en";

export const zh: Dict = {
  toolbar: {
    pickFolder: "选择文件夹",
    scanning: "正在扫描…",
    folder: "文件夹",
    files: "文件",
    keys: "键",
    conflicts: "冲突",
    exportReport: "导出报告",
    mergeIni: "Merged.ini",
    language: "语言",
    info: "关于",
  },
  tabs: {
    files: "文件",
    conflicts: "冲突",
  },
  fileList: {
    searchPlaceholder: "按文件名搜索…",
    onlyConflicts: "仅冲突项",
    filesCount: "文件",
    of: "/",
    nothingFound: "未找到内容",
  },
  conflictList: {
    title: "冲突",
    differingCount: "个值不同",
    sameCount: "个值相同",
    noConflicts: "未发现冲突",
    valuesDiffer: "值不同",
    valuesSame: "值相同",
    winner: "胜出者",
    page: "行",
    uniqueValues: "个不同的值",
  },
  viewer: {
    conflictTooltip: "同时在 {count} 个文件中出现：",
  },
  emptyState: {
    pickFolderTitle: "选择要扫描的文件夹",
    pickFolderDescription:
      "ESPector 将查找所有 .ini 文件，解析并显示模组之间的冲突。",
    pickFileTitle: "从左侧选择一个文件",
    pickFileDescription: "点击列表中的任意 .ini 文件查看其内容。",
  },
  about: {
    close: "关闭",
    version: "版本 {v}",
    description:
      "轻量的 Bethesda 模组工具。扫描 .ini 文件，检测模组之间的冲突，确定胜出值并生成 merged.ini。完全离线运行——无云端、无订阅、无数据收集。",
    support: "支持项目",
    license: "MIT 许可证",
    copyright: "© 2026 ESPector",
  },
  support: {
    title: "支持项目",
    subtitle: "请开发者喝杯咖啡",
    copy: "复制",
    thanks: "感谢您的支持 ❤️",
  },
  errors: {
    prefix: "错误：",
  },
};