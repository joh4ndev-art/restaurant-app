import { setState } from "./state/store.js";
import { canAccess, allowedViews } from "./auth.js";

let current = null;
let initialized = false;

const viewHooks = new Map();

export function registerViewHook(view, hook) {
  viewHooks.set(view, hook);
}

export function navigate(view) {
  // Reject navigation to forbidden views.
  if (!canAccess(view)) {
    const allowed = allowedViews();
    if (allowed.length === 0) return;
    view = allowed[0];
  }

  current = view;

  document.querySelectorAll("[data-view]").forEach((el) => {
    el.classList.toggle("hidden", el.dataset.view !== view);
  });

  document.querySelectorAll("[data-nav]").forEach((btn) => {
    const navView = btn.dataset.nav;
    const allowed = canAccess(navView);
    btn.classList.toggle("hidden", !allowed);
    btn.classList.toggle("active", navView === view);
  });

  setState({ currentView: view });

  if (initialized) {
    const hook = viewHooks.get(view);
    if (typeof hook === "function") hook();
  } else {
    initialized = true;
  }
}

/// Force re-evaluation of the current view's permissions.
/// Called after login/logout.
export function refreshNavigation() {
  const allowed = allowedViews();
  if (allowed.length === 0) return;

  if (!allowed.includes(current)) {
    navigate(allowed[0]);
    return;
  }

  // Current view is still allowed: just update the nav button visibility.
  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.classList.toggle("hidden", !canAccess(btn.dataset.nav));
  });
}

export function initRouter() {
  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.addEventListener("click", () => navigate(btn.dataset.nav));
  });
  const allowed = allowedViews();
  if (allowed.length > 0) navigate(allowed[0]);
}

export function currentView() {
  return current;
}