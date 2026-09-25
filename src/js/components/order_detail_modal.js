import { t } from "../i18n/index.js";

/**
 * Show a modal with the details of an order.
 * Resolves with an action string ('served' | 'cancelled') or `null` if closed.
 */
export function openOrderDetailModal(order) {
  return new Promise((resolve) => {
    const backdrop = document.createElement("div");
    backdrop.className = "modal-backdrop";

    const dialog = document.createElement("div");
    dialog.className = "modal-dialog modal-wide";

    const heading = document.createElement("h3");
    heading.className = "modal-title";
    heading.textContent = t("orders.detail.title", { id: order.id });

    const meta = document.createElement("div");
    meta.className = "order-meta";

    const statusRow = document.createElement("div");
    statusRow.className = "order-meta-row";
    statusRow.innerHTML = `
      <span class="order-meta-label">${t("orders.detail.status")}</span>
      <span class="order-status order-status-${order.status}">
        ${t(`orders.status.${order.status}`)}
      </span>
    `;

    const tableRow = document.createElement("div");
    tableRow.className = "order-meta-row";
    tableRow.innerHTML = `
      <span class="order-meta-label">${t("orders.detail.table")}</span>
      <span>${order.table_id ? `#${order.table_id}` : "—"}</span>
    `;

    const totalRow = document.createElement("div");
    totalRow.className = "order-meta-row";
    totalRow.innerHTML = `
      <span class="order-meta-label">${t("orders.detail.total")}</span>
      <span class="order-total">$${(order.total_cents / 100).toFixed(2)}</span>
    `;

    meta.append(statusRow, tableRow, totalRow);

    const itemsTitle = document.createElement("h4");
    itemsTitle.className = "order-items-title";
    itemsTitle.textContent = t("orders.detail.items");

    const itemsList = document.createElement("div");
    itemsList.className = "order-items-list";

    if (order.items.length === 0) {
      const empty = document.createElement("div");
      empty.className = "order-items-empty";
      empty.textContent = t("orders.detail.no_items");
      itemsList.appendChild(empty);
    } else {
      for (const item of order.items) {
        const row = document.createElement("div");
        row.className = "order-item-row";

        const name = document.createElement("div");
        name.className = "order-item-name";
        name.textContent = `${item.quantity}× ${item.product_name}`;

        const price = document.createElement("div");
        price.className = "order-item-price";
        price.textContent = `$${((item.unit_price_cents * item.quantity) / 100).toFixed(2)}`;

        row.append(name, price);
        itemsList.appendChild(row);
      }
    }

    const actions = document.createElement("div");
    actions.className = "modal-actions";

    const closeBtn = document.createElement("button");
    closeBtn.type = "button";
    closeBtn.className = "btn";
    closeBtn.textContent = t("orders.detail.close");
    closeBtn.addEventListener("click", () => close(null));

    if (order.status === "sent_to_kitchen") {
      const servedBtn = document.createElement("button");
      servedBtn.type = "button";
      servedBtn.className = "btn btn-primary";
      servedBtn.textContent = t("orders.actions.mark_served");
      servedBtn.addEventListener("click", () => close("served"));
      actions.appendChild(servedBtn);
    }

    if (order.status === "served") {
      const payBtn = document.createElement("button");
      payBtn.type = "button";
      payBtn.className = "btn btn-primary";
      payBtn.textContent = t("orders.actions.pay");
      payBtn.addEventListener("click", () => close("pay"));
      actions.appendChild(payBtn);
    }

    if (order.status === "open" || order.status === "sent_to_kitchen") {
      const cancelBtn = document.createElement("button");
      cancelBtn.type = "button";
      cancelBtn.className = "btn btn-danger";
      cancelBtn.textContent = t("orders.actions.cancel");
      cancelBtn.addEventListener("click", () => {
        if (confirm(t("orders.confirm_cancel", { id: order.id }))) {
          close("cancelled");
        }
      });
      actions.appendChild(cancelBtn);
    }

    actions.appendChild(closeBtn);

    dialog.append(heading, meta, itemsTitle, itemsList, actions);
    backdrop.appendChild(dialog);
    document.body.appendChild(backdrop);

    function close(result) {
      backdrop.remove();
      document.removeEventListener("keydown", onKey);
      resolve(result);
    }

    function onKey(event) {
      if (event.key === "Escape") close(null);
    }
    document.addEventListener("keydown", onKey);
  });
}