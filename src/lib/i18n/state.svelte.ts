// Простая локализация: три словаря + реактивная переменная языка.
// Никаких генераторов, watcher'ов и хитрых типов.

import { en, type Dict } from "./en";
import { ru } from "./ru";
import { zh } from "./zh";

export type Locale = "en" | "ru" | "zh";

export const SUPPORTED_LOCALES: Locale[] = ["en", "ru", "zh"];

const DICTS: Record<Locale, Dict> = { en, ru, zh };

// Реактивный контейнер. Мутируем .value, чтобы Svelte не ругался на экспорт.
const localeState = $state({ value: "en" as Locale });
export { localeState as locale };

// Объект переводов — реактивный.
let dict = $state<Dict>(en);

// Возвращает текущий словарь. Компоненты используют как LL().toolbar.pickFolder.
export function LL(): Dict {
  return dict;
}

// Смена языка. Синхронно, потому что словари уже в памяти.
export function changeLocale(locale: Locale): void {
  localeState.value = locale;
  dict = DICTS[locale];
}

// Автоопределение системного языка. Вызывать один раз при старте.
export async function detectSystemLocale(): Promise<void> {
  try {
    const { locale: getOsLocale } = await import("@tauri-apps/plugin-os");
    const systemLocale = await getOsLocale();
    const lang = systemLocale?.split("-")[0].toLowerCase();
    if (lang === "ru") changeLocale("ru");
    else if (lang === "zh") changeLocale("zh");
    else changeLocale("en");
  } catch {
    // Если плагин недоступен — остаёмся на en.
    changeLocale("en");
  }
}