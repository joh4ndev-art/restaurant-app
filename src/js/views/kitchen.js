import { subscribe, setState, getState } from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { ordersApi } from "../api/ipc.js";
import { registerViewHook } from "../router.js";
import { onOrdersChanged } from "../events.js";
import { printingApi } from "../api/ipc.js";

// KDS shows only orders that are physically in the kitchen.
const KDS_STATUSES = ["sent_to_kitchen"];

// Time thresholds (in minutes) for the color coding.
const URGENT_MINUTES = 15;
const WARNING_MINUTES = 5;

// How often the elapsed-time display refreshes (in milliseconds).
const TICK_MS = 10_000;

let tickHandle = null;

// ---------- Helpers ----------

function parseUTC(isoString) {
  // SQLite returns "YYYY-MM-DD HH:MM:SS" in UTC.
  return new Date(isoString.replace(" ", "T") + "Z");
}

function elapsedMinutes(createdAt) {
  const diffMs = Date.now() - parseUTC(createdAt).getTime();
  return Math.max(0, Math.floor(diffMs / 60000));
}

function urgencyClass(minutes) {
  if (minutes >= URGENT_MINUTES) return "urgent";
  if (minutes >= WARNING_MINUTES) return "warning";
  return "fresh";
}

function formatElapsed(minutes) {
  if (minutes < 60) {
    return t("kitchen.elapsed_minutes", { count: minutes });
  }
  const hours = Math.floor(minutes / 60);
  const rest = minutes % 60;
  return t("kitchen.elapsed_hours", { hours, minutes: rest });
}

// ---------- Data loading ----------

async function loadKitchen() {
  setState({ kitchenLoading: true, error: null });

  const res = await ordersApi.listByStatuses(KDS_STATUSES);
  if (!res.ok) {
    setState({ kitchenLoading: false, error: res.error.message });
    return;
  }

  setState({ kitchenOrders: res.data, kitchenLoading: false });
}

// ---------- Actions ----------

async function markServed(order) {
  const res = await ordersApi.markServed(order.id);
  if (!res.ok) {
    alert(t("kitchen.errors.serve_failed", { message: res.error.message }));
    return;
  }
  // No manual reload: the backend will emit "orders:changed"
  // and our subscription will refresh the view.
}

async function printTicket(order) {
  const res = await printingApi.printKitchenTicket(order.id);
  if (!res.ok) {
    alert(t("kitchen.errors.print_failed", { message: res.error.message }));
    return;
  }
  const open = confirm(
    t("printing.ticket_saved", { path: res.data.path })
  );
  if (open) {
    await printingApi.openFile(res.data.path);
  }
}

// ---------- Rendering ----------

function orderCard(order) {
  const minutes = elapsedMinutes(order.created_at);
  const urgency = urgencyClass(minutes);

  const card = document.createElement("article");
  card.className = `kds-card kds-card-${urgency}`;

  // Header: order id + elapsed
  const header = document.createElement("header");
  header.className = "kds-card-header";

  const id = document.createElement("span");
  id.className = "kds-card-id";
  id.textContent = `#${order.id}`;

  const time = document.createElement("span");
  time.className = "kds-card-time";
  time.textContent = formatElapsed(minutes);

  header.append(id, time);

  // Table
  const table = document.createElement("div");
  table.className = "kds-card-table";
  table.textContent = order.table_id
    ? t("kitchen.card.table", { id: order.table_id })
    : t("kitchen.card.no_table");

  // Items
  const list = document.createElement("ul");
  list.className = "kds-card-items";

  for (const item of order.items) {
    if (item.status === "cancelled") continue;

    const li = document.createElement("li");
    li.className = "kds-card-item";

    const qty = document.createElement("span");
    qty.className = "kds-item-qty";
    qty.textContent = `${item.quantity}×`;

    const name = document.createElement("span");
    name.className = "kds-item-name";
    name.textContent = item.product_name;

    li.append(qty, name);

    if (item.notes) {
      const notes = document.createElement("div");
      notes.className = "kds-item-notes";
      notes.textContent = item.notes;
      li.appendChild(notes);
    }

    list.appendChild(li);
  }

  // Actions row
  const actionRow = document.createElement("div");
  actionRow.className = "kds-actions";

  const printBtn = document.createElement("button");
  printBtn.type = "button";
  printBtn.className = "btn kds-print-btn";
  printBtn.textContent = t("kitchen.actions.print");
  printBtn.addEventListener("click", () => printTicket(order));

  const serveBtn = document.createElement("button");
  serveBtn.type = "button";
  serveBtn.className = "btn btn-primary kds-serve-btn";
  serveBtn.textContent = t("kitchen.actions.mark_served");
  serveBtn.addEventListener("click", () => markServed(order));

  actionRow.append(printBtn, serveBtn);

  card.append(header, table, list, actionRow);
  return card;
}

function render(state) {
  const root = document.getElementById("kitchen-container");
  if (!root) return;

  root.innerHTML = "";

  if (state.kitchenLoading && (!state.kitchenOrders || state.kitchenOrders.length === 0)) {
    root.textContent = t("common.loading");
    return;
  }

  if (state.error) {
    root.textContent = `${t("common.error")}: ${state.error}`;
    return;
  }

  const orders = state.kitchenOrders ?? [];

  if (orders.length === 0) {
    const empty = document.createElement("div");
    empty.className = "kds-empty";
    empty.textContent = t("kitchen.empty");
    root.appendChild(empty);
    return;
  }

  // Sort by oldest first: the cook should see the most urgent orders first.
  const sorted = [...orders].sort(
    (a, b) => parseUTC(a.created_at) - parseUTC(b.created_at)
  );

  const grid = document.createElement("div");
  grid.className = "kds-grid";

  for (const order of sorted) {
    grid.appendChild(orderCard(order));
  }

  root.appendChild(grid);
}

// ---------- Entry point ----------

export function mountKitchenView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));
  registerViewHook("kitchen", loadKitchen);

  // Auto-refresh when the backend announces a change.
  onOrdersChanged(() => loadKitchen());

  // Refresh elapsed-time display periodically without hitting the DB.
  if (tickHandle) clearInterval(tickHandle);
  tickHandle = setInterval(() => {
    const state = getState();
    if (state.currentView === "kitchen") {
      render(state);
    }
  }, TICK_MS);

  render(getState());
  loadKitchen();
}