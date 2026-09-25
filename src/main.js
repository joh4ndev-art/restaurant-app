import { initI18n, t, onLocaleChange } from "./js/i18n/index.js";
import { initRouter, registerViewHook, refreshNavigation } from "./js/router.js";
import { getState } from "./js/state/store.js";

import { startEventBridge } from "./js/events.js";

import { mountTablesView } from "./js/views/tables.js";
import { mountPosView } from "./js/views/pos.js";
import { mountOrdersView } from "./js/views/orders.js";
import { mountKitchenView } from "./js/views/kitchen.js";
import { mountProductsView } from "./js/views/products.js";
import { mountReportsView } from "./js/views/reports.js";
import { mountSettingsView, reloadSettingsView } from "./js/views/settings.js";

import { showLogin, hideLogin } from "./js/views/login.js";
import { detectSessionMode, clearSession } from "./js/auth.js";

// ---------- Static translations ----------

function applyStaticTranslations() {
  const state = getState();

  document.title = t("app.title");
  document.getElementById("app-title").textContent = t("app.title");

  // View titles
  document.getElementById("tables-title").textContent = t("tables.title");
  document.getElementById("products-title").textContent = t("products.title");
  document.getElementById("orders-title").textContent = t("orders.title");
  document.getElementById("reports-title").textContent = t("reports.title");
  document.getElementById("kitchen-title").textContent = t("kitchen.title");

  // Primary action buttons
  document.getElementById("add-table-btn").textContent = t("tables.add_button");
  document.getElementById("add-category-btn").textContent = t("products.add_category");
  document.getElementById("add-product-btn").textContent = t("products.add_product");

  // Navigation buttons
  document.querySelector('[data-nav="tables"]').textContent = t("nav.tables");
  document.querySelector('[data-nav="pos"]').textContent = t("nav.pos");
  document.querySelector('[data-nav="orders"]').textContent = t("nav.orders");
  document.querySelector('[data-nav="kitchen"]').textContent = t("nav.kitchen");
  document.querySelector('[data-nav="products"]').textContent = t("nav.products");
  document.querySelector('[data-nav="reports"]').textContent = t("nav.reports");
  document.querySelector('[data-nav="settings"]').textContent = t("nav.settings");

  // Logout button
  const logoutBtn = document.getElementById("logout-btn");
  if (logoutBtn) logoutBtn.textContent = t("login.logout");

  document.documentElement.lang = state.locale ?? "en";
}

// ---------- Login overlay visibility ----------

function updateLoginVisibility() {
  const { session } = getState();
  const logoutBtn = document.getElementById("logout-btn");

  if (session.isFreeMode || session.user) {
    hideLogin();
    if (session.user) {
      logoutBtn.classList.remove("hidden");
    } else {
      logoutBtn.classList.add("hidden");
    }
    refreshNavigation();

    const firstAllowedNav = document.querySelector(
      ".nav-btn[data-nav]:not(.hidden)"
    );
    if (firstAllowedNav && !getState().currentView) {
      firstAllowedNav.click();
    } else {
      const { navigate } = { navigate: null };
      // If a view is already set, just re-navigate to refresh state.
      const current = getState().currentView;
      if (current) {
        // Defer to router via public API.
        import("./js/router.js").then(({ navigate }) => navigate(current));
      }
    }
  } else {
    showLogin();
    logoutBtn.classList.add("hidden");
  }
}

// ---------- Bootstrap ----------

window.addEventListener("DOMContentLoaded", async () => {
  await initI18n();
  await startEventBridge();

  applyStaticTranslations();

  // Mount all views. They register their own router hooks.
  mountTablesView();
  mountPosView();
  mountOrdersView();
  mountKitchenView();
  mountProductsView();
  mountReportsView();
  mountSettingsView();

  // Register the Settings reload hook (settings doesn't register itself).
  registerViewHook("settings", reloadSettingsView);

  // Initialize router AFTER hooks are registered.
  initRouter();

  // Language change handler.
  onLocaleChange((locale) => {
    applyStaticTranslations();
    document.documentElement.lang = locale;
  });

  // Logout button.
  document.getElementById("logout-btn").addEventListener("click", () => {
    if (!confirm(t("login.confirm_logout"))) return;
    clearSession();
    updateLoginVisibility();
  });

  // Expose to login.js so it can update UI after a successful login.
  window.__updateLoginVisibility = updateLoginVisibility;

  // Determine startup mode and show login if needed.
  await detectSessionMode();
  updateLoginVisibility();
});