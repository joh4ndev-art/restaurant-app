import { setState } from "./state/store.js";

let current = "tables";

export function navigate(view) {
  current = view;

  document.querySelectorAll("[data-view]").forEach((el) => {
    el.classList.toggle("hidden", el.dataset.view !== view);
  });

  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.classList.toggle("active", btn.dataset.nav === view);
  });

  setState({ currentView: view });
}

export function initRouter() {
  document.querySelectorAll("[data-nav]").forEach((btn) => {
    btn.addEventListener("click", () => navigate(btn.dataset.nav));
  });
  navigate("tables");
}