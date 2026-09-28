import { t } from "../i18n/index.js";
import { paymentsApi, printingApi } from "../api/ipc.js";

const METHODS = ["cash", "card", "other"];

function formatPrice(cents) {
  return `$${(cents / 100).toFixed(2)}`;
}

function parseAmountToCents(str) {
  const value = parseFloat(str);
  if (!Number.isFinite(value) || value < 0) return null;
  return Math.round(value * 100);
}

/**
 * Open the payment modal for an order.
 * Resolves with "paid" when the order is fully paid, or null if closed.
 */
export function openPaymentModal(orderId) {
  return new Promise((resolve) => {
    const backdrop = document.createElement("div");
    backdrop.className = "modal-backdrop";

    const dialog = document.createElement("div");
    dialog.className = "modal-dialog modal-wide";

    backdrop.appendChild(dialog);
    document.body.appendChild(backdrop);

    let summary = null;
    let method = "cash";
    let amountStr = "";
    let submitting = false;

    // ---------- Lifecycle ----------

    function close(result) {
      backdrop.remove();
      document.removeEventListener("keydown", onKey);
      resolve(result);
    }

    function onKey(event) {
      if (event.key === "Escape" && !submitting) close(null);
    }
    document.addEventListener("keydown", onKey);

    async function loadSummary() {
      const res = await paymentsApi.summary(orderId);
      if (!res.ok) {
        alert(t("payments.errors.load_failed", { message: res.error.message }));
        close(null);
        return;
      }
      summary = res.data;

      if (summary.remaining_cents <= 0) {
        close("paid");
        return;
      }

      amountStr = (summary.remaining_cents / 100).toFixed(2);
      render();
    }

    async function submitPayment() {
      if (submitting) return;
      const entered = parseAmountToCents(amountStr);
      if (entered === null || entered <= 0) {
        alert(t("payments.errors.invalid_amount"));
        return;
      }

      // For cash: the customer may hand over more than the total.
      // We record only what is owed; the rest is change.
      const effective = Math.min(entered, summary.remaining_cents);

      submitting = true;
      const res = await paymentsApi.pay({
        order_id: orderId,
        method,
        amount_cents: effective,
      });
      submitting = false;

      if (!res.ok) {
        alert(t("payments.errors.pay_failed", { message: res.error.message }));
        return;
      }

      summary = res.data;

      if (summary.remaining_cents <= 0) {
        // Order fully paid. Offer to print the receipt before closing.
        await offerPrintReceipt();
        close("paid");
        return;
      }

      amountStr = (summary.remaining_cents / 100).toFixed(2);
      render();
    }

    async function offerPrintReceipt() {
      if (!confirm(t("printing.print_receipt_prompt"))) return;

      const printRes = await printingApi.printReceipt(orderId);
      if (!printRes.ok) {
        alert(t("printing.errors.receipt_failed", { message: printRes.error.message }));
        return;
      }
      const open = confirm(t("printing.receipt_saved", { path: printRes.data.path }));
      if (open) {
        await printingApi.openFile(printRes.data.path);
      }
    }

    // ---------- Rendering ----------

    function render() {
      dialog.innerHTML = "";

      const heading = document.createElement("h3");
      heading.className = "modal-title";
      heading.textContent = t("payments.title", { id: orderId });

      const info = document.createElement("div");
      info.className = "payment-info";

      info.appendChild(summaryRow(t("payments.total"), formatPrice(summary.total_cents)));
      info.appendChild(summaryRow(t("payments.paid"), formatPrice(summary.paid_cents)));
      info.appendChild(
        summaryRow(
          t("payments.remaining"),
          formatPrice(summary.remaining_cents),
          "payment-remaining"
        )
      );

      const listTitle = document.createElement("h4");
      listTitle.className = "order-items-title";
      listTitle.textContent = t("payments.list_title");

      const list = document.createElement("div");
      list.className = "payment-list";

      if (summary.payments.length === 0) {
        const empty = document.createElement("div");
        empty.className = "order-items-empty";
        empty.textContent = t("payments.no_payments");
        list.appendChild(empty);
      } else {
        for (const p of summary.payments) {
          const row = document.createElement("div");
          row.className = "payment-row";

          const methodLabel = document.createElement("span");
          methodLabel.textContent = t(`payments.methods.${p.method}`);

          const amount = document.createElement("span");
          amount.className = "payment-row-amount";
          amount.textContent = formatPrice(p.amount_cents);

          row.append(methodLabel, amount);
          list.appendChild(row);
        }
      }

      const formTitle = document.createElement("h4");
      formTitle.className = "order-items-title";
      formTitle.textContent = t("payments.new_payment");

      const methodRow = document.createElement("div");
      methodRow.className = "payment-method-row";

      for (const m of METHODS) {
        const btn = document.createElement("button");
        btn.type = "button";
        btn.className = "pos-cat-btn" + (m === method ? " active" : "");
        btn.textContent = t(`payments.methods.${m}`);
        btn.addEventListener("click", () => {
          method = m;
          render();
        });
        methodRow.appendChild(btn);
      }

      const amountField = document.createElement("label");
      amountField.className = "modal-field";

      const amountLabel = document.createElement("span");
      amountLabel.textContent = t("payments.amount");

      const amountInput = document.createElement("input");
      amountInput.type = "number";
      amountInput.step = "0.01";
      amountInput.min = "0.01";
      amountInput.value = amountStr;
      amountInput.addEventListener("input", () => {
        amountStr = amountInput.value;
        updateChange();
      });

      amountField.append(amountLabel, amountInput);

      const changeRow = document.createElement("div");
      changeRow.className = "payment-change";
      changeRow.id = "payment-change-row";

      const actions = document.createElement("div");
      actions.className = "modal-actions";

      const closeBtn = document.createElement("button");
      closeBtn.type = "button";
      closeBtn.className = "btn";
      closeBtn.textContent = t("payments.close");
      closeBtn.addEventListener("click", () => close(null));

      const payBtn = document.createElement("button");
      payBtn.type = "button";
      payBtn.className = "btn btn-primary";
      payBtn.textContent = t("payments.add_payment");
      payBtn.addEventListener("click", submitPayment);

      actions.append(closeBtn, payBtn);

      dialog.append(heading, info, listTitle, list, formTitle, methodRow, amountField, changeRow, actions);

      updateChange();
      amountInput.focus();
      amountInput.select();

      function updateChange() {
        const entered = parseAmountToCents(amountStr);
        changeRow.innerHTML = "";
        if (entered === null || entered <= summary.remaining_cents) return;
        const change = entered - summary.remaining_cents;
        changeRow.textContent = t("payments.change", { amount: formatPrice(change) });
      }
    }

    function summaryRow(label, value, valueClass) {
      const row = document.createElement("div");
      row.className = "payment-info-row";

      const lbl = document.createElement("span");
      lbl.className = "payment-info-label";
      lbl.textContent = label;

      const val = document.createElement("span");
      val.className = "payment-info-value" + (valueClass ? ` ${valueClass}` : "");
      val.textContent = value;

      row.append(lbl, val);
      return row;
    }

    // ---------- Go ----------

    loadSummary();
  });
}