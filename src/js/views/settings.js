import { t, getLocale, getSupportedLocales, setLocale } from "../i18n/index.js";
import { setState } from "../state/store.js";

const LOCALE_LABELS = {
  en: "English",
  es: "Español",
};

function render(root) {
  root.innerHTML = "";

  const title = document.createElement("h2");
  title.textContent = t("settings.title");

  const list = document.createElement("div");
  list.className = "settings-list";

  const item = document.createElement("div");
  item.className = "setting-item";

  const info = document.createElement("div");

  const label = document.createElement("div");
  label.className = "setting-label";
  label.textContent = t("settings.language.label");

  const desc = document.createElement("div");
  desc.className = "setting-desc";
  desc.textContent = t("settings.language.description");

  info.append(label, desc);

  const select = document.createElement("select");
  select.className = "setting-select";
  select.id = "locale-select";

  for (const loc of getSupportedLocales()) {
    const opt = document.createElement("option");
    opt.value = loc;
    opt.textContent = LOCALE_LABELS[loc] ?? loc;
    if (loc === getLocale()) opt.selected = true;
    select.appendChild(opt);
  }

  select.addEventListener("change", async () => {
    await setLocale(select.value);
    // Notify the rest of the UI so it re-renders with the new locale.
    setState({ locale: select.value });
    render(root);
  });

  item.append(info, select);
  list.appendChild(item);

  root.append(title, list);
}

export function mountSettingsView() {
  const root = document.getElementById("view-settings");
  if (!root) return;

  render(root);
}