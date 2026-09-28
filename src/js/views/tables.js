import { subscribe, setState, getState } from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { tablesApi } from "../api/ipc.js";
import { ordersApi } from "../api/ipc.js";
import { openOrderDetailModal } from "../components/order_detail_modal.js";
import { openPaymentModal } from "../components/payment_modal.js";
import { registerViewHook } from "../router.js";
import { onOrdersChanged } from "../events.js";

// Status cycle when clicking a table.
const STATUS_CYCLE = {
  free: "occupied",
  occupied: "cleaning",
  cleaning: "free",
  reserved: "occupied",
};

// ---------- Data loading ----------

async function loadTables() {
  setState({ loading: true, error: null });
  const result = await tablesApi.list();

  if (!result.ok) {
    setState({ loading: false, error: result.error.message });
    return;
  }

  setState({ tables: result.data, loading: false });
}

async function createTable() {
  const name = prompt(t("tables.prompts.name"));
  if (!name) return;

  const zone = prompt(t("tables.prompts.zone")) || null;
  const capacityStr = prompt(t("tables.prompts.capacity"), "4");
  const capacity = parseInt(capacityStr, 10);

  if (!Number.isInteger(capacity) || capacity <= 0) {
    alert(t("tables.errors.capacity_invalid"));
    return;
  }

  const result = await tablesApi.create({ name, zone, capacity });
  if (!result.ok) {
    alert(t("tables.errors.create_failed", { message: result.error.message }));
    return;
  }

  await loadTables();
}


async function handleTableClick(table) {
  // If the table has an active order, open its detail modal.
  if (table.status === "occupied") {
    const res = await ordersApi.getActiveForTable(table.id);
    if (!res.ok) {
      alert(t("tables.errors.load_order_failed", { message: res.error.message }));
      return;
    }

    if (res.data) {
      const order = res.data;
      const action = await openOrderDetailModal(order);
      if (!action) return;

      if (action === "served") {
        await ordersApi.markServed(order.id);
      } else if (action === "cancelled") {
        await ordersApi.cancel(order.id);
      } else if (action === "pay") {
        const paid = await openPaymentModal(order.id);
        if (paid !== "paid") return;
      }
      await loadTables();
      return;
    }
    // No active order? Fall through to the default status cycle.
  }

  // Default behavior: cycle status on click.
  const next = STATUS_CYCLE[table.status] ?? "free";
  const result = await tablesApi.updateStatus(table.id, next);
  if (!result.ok) {
    alert(t("tables.errors.update_failed", { message: result.error.message }));
    return;
  }
  await loadTables();
}

// ---------- Rendering ----------

function tableCard(table) {
  const card = document.createElement("button");
  card.type = "button";
  card.className = `table-card status-${table.status}`;

  const name = document.createElement("span");
  name.className = "table-name";
  name.textContent = table.name;

  const meta = document.createElement("span");
  meta.className = "table-meta";
  const statusLabel = t(`tables.status.${table.status}`);
  const zone = table.zone ? ` · ${table.zone}` : "";
  meta.textContent = `${statusLabel}${zone} · ${table.capacity}p`;

  const del = document.createElement("span");
  del.className = "table-delete";
  del.textContent = "×";
  del.title = t("tables.confirm_delete", { name: table.name });
  del.addEventListener("click", (event) => {
    event.stopPropagation();
    deleteTable(table);

    if (table.status === "occupied") {
      const hint = document.createElement("span");
      hint.className = "table-hint";
      hint.textContent = t("tables.click_to_view_order");
      card.appendChild(hint);
    }
  });

  card.append(name, meta, del);
  card.addEventListener("click", () => handleTableClick(table));
  return card;
}

function render(state) {
  const root = document.getElementById("tables-grid");
  if (!root) return;

  root.innerHTML = "";

  if (state.loading) {
    root.textContent = t("common.loading");
    return;
  }

  if (state.error) {
    root.textContent = `${t("common.error")}: ${state.error}`;
    return;
  }

  if (state.tables.length === 0) {
    root.textContent = t("tables.empty");
    return;
  }

  for (const table of state.tables) {
    root.appendChild(tableCard(table));
  }
}

// ---------- Entry point ----------

export function mountTablesView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));

  document
    .getElementById("add-table-btn")
    ?.addEventListener("click", createTable);

  registerViewHook("tables", loadTables);
  onOrdersChanged(() => loadTables());

  render(getState());
  loadTables();
}