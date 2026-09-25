import { setState } from "./state/store.js";

let current = "tables";
let initialized = false;

// Registry of reload hooks, one per view.
// Each view registers a function that (re)loads its own data.
const viewHooks = new Map();

export function registerViewHook(view, hook) {
  viewHooks.set(view, hook);
}

export function navigate(view) {
  current = view;

  document.querySelectorAll("[data-view]").forEach((el) => {
    el.classList.toggle("hidden", el.dataset.view !== view);
  });

  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.classList.toggle("active", btn.dataset.nav === view);
  });

  setState({ currentView: view });

  // On the very first navigation, skip the hook: the view
  // already loaded its data during mount. Subsequent navigations
  // must refresh to avoid stale data.
  if (initialized) {
    const hook = viewHooks.get(view);
    if (typeof hook === "function") hook();
  } else {
    initialized = true;
  }
}

export function initRouter() {
  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.addEventListener("click", () => navigate(btn.dataset.nav));
  });
  navigate("tables");
}