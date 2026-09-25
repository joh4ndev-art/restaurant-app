import { initI18n, t, onLocaleChange } from "./js/i18n/index.js";
import { initRouter } from "./js/router.js";
import { mountTablesView } from "./js/views/tables.js";
import { mountPosView } from "./js/views/pos.js";
import { mountOrdersView } from "./js/views/orders.js";
import { mountProductsView } from "./js/views/products.js";
import { mountSettingsView } from "./js/views/settings.js";
import { getState } from "./js/state/store.js";
import { mountReportsView } from "./js/views/reports.js";

function applyStaticTranslations() {
  const state = getState();

  document.title = t("app.title");
  document.getElementById("app-title").textContent = t("app.title");
  document.getElementById("tables-title").textContent = t("tables.title");
  document.getElementById("products-title").textContent = t("products.title");
  document.getElementById("add-table-btn").textContent = t("tables.add_button");
  document.getElementById("add-category-btn").textContent = t("products.add_category");
  document.getElementById("add-product-btn").textContent = t("products.add_product");

  document.querySelector('[data-nav="tables"]').textContent = t("nav.tables");
  document.querySelector('[data-nav="products"]').textContent = t("nav.products");
  document.querySelector('[data-nav="settings"]').textContent = t("nav.settings");

  document.documentElement.lang = state.locale ?? "en";
}

window.addEventListener("DOMContentLoaded", async () => {
  await initI18n();

  applyStaticTranslations();
  mountTablesView();
  mountPosView();
  mountOrdersView();
  mountReportsView();
  mountProductsView();
  mountSettingsView();
  initRouter();

  onLocaleChange((locale) => {
    applyStaticTranslations();
    document.documentElement.lang = locale;
    document.querySelector('[data-nav="pos"]').textContent = t("nav.pos");
    document.getElementById("orders-title").textContent = t("orders.title");
    document.querySelector('[data-nav="orders"]').textContent = t("nav.orders");
    document.getElementById("reports-title").textContent = t("reports.title");
    document.querySelector('[data-nav="reports"]').textContent = t("nav.reports");
  });
});