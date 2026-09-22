import { initI18n, t, onLocaleChange } from "./js/i18n/index.js";
import { initRouter } from "./js/router.js";
import { mountTablesView } from "./js/views/tables.js";
import { mountSettingsView } from "./js/views/settings.js";
import { getState } from "./js/state/store.js";

function applyStaticTranslations() {
  const state = getState();

  document.title = t("app.title");
  document.getElementById("app-title").textContent = t("app.title");
  document.getElementById("tables-title").textContent = t("tables.title");
  document.getElementById("add-table-btn").textContent = t("tables.add_button");

  const navTables = document.querySelector('[data-nav="tables"]');
  const navSettings = document.querySelector('[data-nav="settings"]');
  if (navTables) navTables.textContent = t("nav.tables");
  if (navSettings) navSettings.textContent = t("nav.settings");

  document.documentElement.lang = state.locale ?? "en";
}

window.addEventListener("DOMContentLoaded", async () => {
  await initI18n();

  applyStaticTranslations();
  mountTablesView();
  mountSettingsView();
  initRouter();

  onLocaleChange((locale) => {
    applyStaticTranslations();
    document.documentElement.lang = locale;
  });
});