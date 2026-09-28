import { subscribe, setState, getState } from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { ordersApi } from "../api/ipc.js";
import { openOrderDetailModal } from "../components/order_detail_modal.js";
import { openPaymentModal } from "../components/payment_modal.js";
import { registerViewHook } from "../router.js";
import { onOrdersChanged } from "../events.js";
import { printingApi } from "../api/ipc.js";

// Statuses considered "active" for this view.
const ACTIVE_STATUSES = ["open", "sent_to_kitchen", "served"];

// ---------- Data loading ----------

async function loadOrders() {
  setState({ productsLoading: true, error: null });
  const res = await ordersApi.listByStatuses(ACTIVE_STATUSES);

  if (!res.ok) {
    setState({ productsLoading: false, error: res.error.message });
    return;
  }

  setState({ orders: res.data, productsLoading: false });
}


// ---------- Actions ----------

async function handleOrder(order) {
  const action = await openOrderDetailModal(order);
  if (!action) return;

  if (action === "served") {
    const res = await ordersApi.markServed(order.id);
    if (!res.ok) {
      alert(t("orders.errors.serve_failed", { message: res.error.message }));
      return;
    }
  } else if (action === "cancelled") {
    const res = await ordersApi.cancel(order.id);
    if (!res.ok) {
      alert(t("orders.errors.cancel_failed", { message: res.error.message }));
      return;
    }
  } else if (action === "pay") {
    const paid = await openPaymentModal(order.id);
    if (paid !== "paid") return;
  }

  await loadOrders();
}

// ---------- Rendering ----------

function formatTime(isoString) {
  // SQLite returns "YYYY-MM-DD HH:MM:SS" in UTC.
  const date = new Date(isoString.replace(" ", "T") + "Z");
  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function orderCard(order) {
  const card = document.createElement("button");
  card.type = "button";
  card.className = `order-card order-status-${order.status}`;

  const header = document.createElement("div");
  header.className = "order-card-header";

  const id = document.createElement("span");
  id.className = "order-card-id";
  id.textContent = `#${order.id}`;

  const time = document.createElement("span");
  time.className = "order-card-time";
  time.textContent = formatTime(order.created_at);

  header.append(id, time);

  const table = document.createElement("div");
  table.className = "order-card-table";
  table.textContent = order.table_id
    ? t("orders.card.table", { id: order.table_id })
    : t("orders.card.no_table");

  const items = document.createElement("div");
  items.className = "order-card-items";
  items.textContent = t("orders.card.items", { count: order.items.length });

  const footer = document.createElement("div");
  footer.className = "order-card-footer";

  const status = document.createElement("span");
  status.className = `order-status order-status-${order.status}`;
  status.textContent = t(`orders.status.${order.status}`);

  const total = document.createElement("span");
  total.className = "order-card-total";
  total.textContent = `$${(order.total_cents / 100).toFixed(2)}`;

  footer.append(status, total);

  card.append(header, table, items, footer);
  card.addEventListener("click", () => handleOrder(order));
  return card;
}

function render(state) {
  const root = document.getElementById("orders-container");
  if (!root) return;

  root.innerHTML = "";

  if (state.productsLoading) {
    root.textContent = t("common.loading");
    return;
  }

  if (state.error) {
    root.textContent = `${t("common.error")}: ${state.error}`;
    return;
  }

  if (!state.orders || state.orders.length === 0) {
    root.textContent = t("orders.empty");
    return;
  }

  const grid = document.createElement("div");
  grid.className = "orders-grid";

  for (const order of state.orders) {
    grid.appendChild(orderCard(order));
  }

  root.appendChild(grid);
}

// ---------- Entry point ----------

export function mountOrdersView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));
  registerViewHook("orders", loadOrders);
  onOrdersChanged(() => loadOrders());

  render(getState());
  loadOrders();
}