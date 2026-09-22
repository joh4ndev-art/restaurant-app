import { settingsApi } from "../api/ipc.js";

const SUPPORTED = ["en", "es"];
const DEFAULT_LOCALE = "en";
const STORAGE_KEY = "locale";

const loaders = {
  en: () => import("./locales/en.json", { with: { type: "json" } }),
  es: () => import("./locales/es.json", { with: { type: "json" } }),
};

let current = DEFAULT_LOCALE;
let messages = {};
const listeners = new Set();

/// Load the saved locale from SQLite (falls back to default).
export async function initI18n() {
  const result = await settingsApi.get(STORAGE_KEY);
  const saved = result.ok ? result.data : null;
  const initial = SUPPORTED.includes(saved) ? saved : DEFAULT_LOCALE;
  await loadLocale(initial);
}

/// Load a locale into memory and notify subscribers.
export async function loadLocale(locale) {
  if (!SUPPORTED.includes(locale)) {
    throw new Error(`Unsupported locale: ${locale}`);
  }
  const mod = await loaders[locale]();
  messages = mod.default;
  current = locale;
  listeners.forEach((fn) => fn(current));
}

/// Change locale and persist it.
export async function setLocale(locale) {
  await loadLocale(locale);
  await settingsApi.set(STORAGE_KEY, locale);
}

/// Translate a key. Supports `{placeholder}` substitution.
export function t(key, vars = {}) {
  const value = key
    .split(".")
    .reduce((obj, k) => (obj == null ? undefined : obj[k]), messages);

  if (typeof value !== "string") return key;

  return value.replace(/\{(\w+)\}/g, (_, name) =>
    vars[name] !== undefined ? vars[name] : `{${name}}`
  );
}

export function getLocale() {
  return current;
}

export function getSupportedLocales() {
  return [...SUPPORTED];
}

export function onLocaleChange(fn) {
  listeners.add(fn);
  return () => listeners.delete(fn);
}