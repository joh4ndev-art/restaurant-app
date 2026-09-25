import { subscribe, setState, getState } from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { reportsApi } from "../api/ipc.js";
import { registerViewHook } from "../router.js";
import {
  today,
  daysAgo,
  startOfMonth,
  formatDisplayDate,
} from "../utils/dates.js";

// ---------- State helpers ----------

const DEFAULT_RANGE = { from: today(), to: today(), preset: "today" };

function ensureRangeInStore() {
  const state = getState();
  if (!state.reportsRange) {
    setState({ reportsRange: DEFAULT_RANGE, reports: null, reportsLoading: false });
  }
}

// ---------- Data loading ----------

async function loadReport() {
  ensureRangeInStore();
  const { reportsRange } = getState();

  setState({ reportsLoading: true, error: null });

  const res = await reportsApi.generate({
    from: reportsRange.from,
    to: reportsRange.to,
  });

  if (!res.ok) {
    setState({ reportsLoading: false, error: res.error.message });
    return;
  }

  setState({ reports: res.data, reportsLoading: false });
}

// ---------- Preset handling ----------

function applyPreset(preset) {
  let from, to;
  switch (preset) {
    case "today":
      from = today();
      to = today();
      break;
    case "last7":
      from = daysAgo(6);
      to = today();
      break;
    case "month":
      from = startOfMonth();
      to = today();
      break;
    case "custom":
      // Keep existing dates; UI exposes date inputs.
      const current = getState().reportsRange;
      from = current.from;
      to = current.to;
      break;
    default:
      return;
  }
  setState({ reportsRange: { from, to, preset } });
  loadReport();
}

// ---------- Rendering ----------

function formatPrice(cents) {
  return `$${(cents / 100).toFixed(2)}`;
}

function renderFilters(state, root) {
  const container = document.createElement("div");
  container.className = "reports-filters";

  const presets = [
    { key: "today", label: t("reports.presets.today") },
    { key: "last7", label: t("reports.presets.last7") },
    { key: "month", label: t("reports.presets.month") },
    { key: "custom", label: t("reports.presets.custom") },
  ];

  for (const p of presets) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "pos-cat-btn" + (state.reportsRange.preset === p.key ? " active" : "");
    btn.textContent = p.label;
    btn.addEventListener("click", () => applyPreset(p.key));
    container.appendChild(btn);
  }

  // Custom date inputs (always visible, enabled when preset === 'custom')
  const dateRow = document.createElement("div");
  dateRow.className = "reports-date-row" + (state.reportsRange.preset === "custom" ? "" : " disabled");

  const fromLabel = document.createElement("label");
  fromLabel.textContent = t("reports.from");

  const fromInput = document.createElement("input");
  fromInput.type = "date";
  fromInput.value = state.reportsRange.from;
  fromInput.disabled = state.reportsRange.preset !== "custom";
  fromInput.addEventListener("change", () => {
    setState({ reportsRange: { ...state.reportsRange, from: fromInput.value } });
    loadReport();
  });

  const toLabel = document.createElement("label");
  toLabel.textContent = t("reports.to");

  const toInput = document.createElement("input");
  toInput.type = "date";
  toInput.value = state.reportsRange.to;
  toInput.disabled = state.reportsRange.preset !== "custom";
  toInput.addEventListener("change", () => {
    setState({ reportsRange: { ...state.reportsRange, to: toInput.value } });
    loadReport();
  });

  fromLabel.appendChild(fromInput);
  toLabel.appendChild(toInput);
  dateRow.append(fromLabel, toLabel);

  root.append(container, dateRow);
}

function renderSummaryCards(state, root) {
  const bundle = state.reports;
  if (!bundle) return;

  const grid = document.createElement("div");
  grid.className = "reports-summary-grid";

  const cards = [
    {
      label: t("reports.summary.total_sales"),
      value: formatPrice(bundle.summary.total_cents),
      accent: true,
    },
    {
      label: t("reports.summary.orders"),
      value: String(bundle.summary.order_count),
    },
    {
      label: t("reports.summary.average_ticket"),
      value: formatPrice(bundle.summary.average_ticket_cents),
    },
    {
      label: t("reports.summary.payments"),
      value: String(bundle.summary.payment_count),
    },
  ];

  for (const c of cards) {
    const card = document.createElement("div");
    card.className = "reports-summary-card";

    const label = document.createElement("div");
    label.className = "reports-summary-label";
    label.textContent = c.label;

    const value = document.createElement("div");
    value.className = "reports-summary-value" + (c.accent ? " accent" : "");
    value.textContent = c.value;

    card.append(label, value);
    grid.appendChild(card);
  }

  root.appendChild(grid);
}

function renderTopProducts(state, root) {
  const bundle = state.reports;
  if (!bundle) return;

  const section = document.createElement("section");
  section.className = "reports-section";

  const title = document.createElement("h3");
  title.className = "reports-section-title";
  title.textContent = t("reports.top_products.title");

  section.appendChild(title);

  if (bundle.top_products.length === 0) {
    const empty = document.createElement("div");
    empty.className = "reports-empty";
    empty.textContent = t("reports.top_products.empty");
    section.appendChild(empty);
    root.appendChild(section);
    return;
  }

  const table = document.createElement("div");
  table.className = "reports-table";

  // Header
  const header = document.createElement("div");
  header.className = "reports-table-row header";
  header.innerHTML = `
    <span>${t("reports.top_products.product")}</span>
    <span>${t("reports.top_products.quantity")}</span>
    <span>${t("reports.top_products.revenue")}</span>
  `;
  table.appendChild(header);

  for (const p of bundle.top_products) {
    const row = document.createElement("div");
    row.className = "reports-table-row";

    const name = document.createElement("span");
    name.textContent = p.product_name;

    const qty = document.createElement("span");
    qty.textContent = String(p.quantity);

    const revenue = document.createElement("span");
    revenue.className = "reports-revenue";
    revenue.textContent = formatPrice(p.revenue_cents);

    row.append(name, qty, revenue);
    table.appendChild(row);
  }

  section.appendChild(table);
  root.appendChild(section);
}

function renderSalesByDay(state, root) {
  const bundle = state.reports;
  if (!bundle) return;

  const section = document.createElement("section");
  section.className = "reports-section";

  const title = document.createElement("h3");
  title.className = "reports-section-title";
  title.textContent = t("reports.sales_by_day.title");

  section.appendChild(title);

  if (bundle.sales_by_day.length === 0) {
    const empty = document.createElement("div");
    empty.className = "reports-empty";
    empty.textContent = t("reports.sales_by_day.empty");
    section.appendChild(empty);
    root.appendChild(section);
    return;
  }

  const table = document.createElement("div");
  table.className = "reports-table";

  const header = document.createElement("div");
  header.className = "reports-table-row header";
  header.innerHTML = `
    <span>${t("reports.sales_by_day.date")}</span>
    <span>${t("reports.sales_by_day.orders")}</span>
    <span>${t("reports.sales_by_day.total")}</span>
  `;
  table.appendChild(header);

  for (const d of bundle.sales_by_day) {
    const row = document.createElement("div");
    row.className = "reports-table-row";

    const date = document.createElement("span");
    date.textContent = formatDisplayDate(d.date);

    const orders = document.createElement("span");
    orders.textContent = String(d.order_count);

    const total = document.createElement("span");
    total.className = "reports-revenue";
    total.textContent = formatPrice(d.total_cents);

    row.append(date, orders, total);
    table.appendChild(row);
  }

  section.appendChild(table);
  root.appendChild(section);
}

function render(state) {
  const root = document.getElementById("reports-container");
  if (!root) return;

  root.innerHTML = "";

  ensureRangeInStore();

  renderFilters(state, root);

  if (state.reportsLoading) {
    const loading = document.createElement("div");
    loading.className = "reports-loading";
    loading.textContent = t("common.loading");
    root.appendChild(loading);
    return;
  }

  if (state.error) {
    const err = document.createElement("div");
    err.className = "reports-error";
    err.textContent = `${t("common.error")}: ${state.error}`;
    root.appendChild(err);
    return;
  }

  renderSummaryCards(state, root);
  renderTopProducts(state, root);
  renderSalesByDay(state, root);
}

// ---------- Entry point ----------

export function mountReportsView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));

  registerViewHook("reports", loadReport);

  ensureRangeInStore();
  render(getState());
  loadReport();
}