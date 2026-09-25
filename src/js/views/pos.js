import {
  subscribe,
  setState,
  getState,
  posSetTable,
  posSetCategory,
  posAddToCart,
  posDecrementFromCart,
  posRemoveFromCart,
  posClearCart,
  posCartTotal,
} from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { categoriesApi, productsApi, ordersApi, tablesApi } from "../api/ipc.js";
import { registerViewHook } from "../router.js";

// ---------- Helpers ----------

function formatPrice(cents) {
  return `$${(cents / 100).toFixed(2)}`;
}

// ---------- Data loading ----------

async function loadAll() {
  setState({ productsLoading: true, error: null });

  const [cats, prods, tbls] = await Promise.all([
    categoriesApi.list(),
    productsApi.list(),
    tablesApi.list(),
  ]);

  if (!cats.ok || !prods.ok || !tbls.ok) {
    setState({
      productsLoading: false,
      error:
        cats.error?.message ??
        prods.error?.message ??
        tbls.error?.message ??
        "unknown error",
    });
    return;
  }

  setState({
    categories: cats.data,
    products: prods.data,
    tables: tbls.data,
    productsLoading: false,
  });
}

// ---------- Actions ----------

async function sendToKitchen() {
  const state = getState();
  const { selectedTableId, cart } = state.pos;

  if (!selectedTableId) {
    alert(t("pos.errors.no_table"));
    return;
  }
  if (cart.length === 0) {
    alert(t("pos.errors.empty_cart"));
    return;
  }

  const input = {
    table_id: selectedTableId,
    notes: null,
    items: cart.map((item) => ({
      product_id: item.productId,
      quantity: item.quantity,
      notes: item.notes,
    })),
  };

  const res = await ordersApi.send(input);
  if (!res.ok) {
    alert(t("pos.errors.send_failed", { message: res.error.message }));
    return;
  }

  posClearCart();
  posSetTable(null);
  await loadAll();

  alert(
    t("pos.order_sent", {
      id: res.data.id,
      total: formatPrice(res.data.total_cents),
    })
  );
}

// ---------- Rendering ----------

function renderTableSelector(state, container) {
  container.innerHTML = "";

  const label = document.createElement("label");
  label.className = "pos-label";
  label.textContent = t("pos.select_table");
  label.htmlFor = "pos-table-select";

  const select = document.createElement("select");
  select.id = "pos-table-select";
  select.className = "pos-select";

  const placeholder = document.createElement("option");
  placeholder.value = "";
  placeholder.textContent = t("pos.select_table_placeholder");
  select.appendChild(placeholder);

  const freeTables = state.tables.filter((tbl) => tbl.status === "free");
  for (const tbl of freeTables) {
    const opt = document.createElement("option");
    opt.value = String(tbl.id);
    opt.textContent = tbl.zone ? `${tbl.name} · ${tbl.zone}` : tbl.name;
    if (state.pos.selectedTableId === tbl.id) opt.selected = true;
    select.appendChild(opt);
  }

  select.addEventListener("change", () => {
    const value = select.value ? parseInt(select.value, 10) : null;
    posSetTable(value);
  });

  container.append(label, select);
}

function renderCategoryFilter(state, container) {
  container.innerHTML = "";

  const all = document.createElement("button");
  all.type = "button";
  all.className = "pos-cat-btn" + (state.pos.selectedCategoryId === null ? " active" : "");
  all.textContent = t("pos.all_categories");
  all.addEventListener("click", () => posSetCategory(null));
  container.appendChild(all);

  for (const cat of state.categories) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "pos-cat-btn" + (state.pos.selectedCategoryId === cat.id ? " active" : "");
    btn.textContent = cat.name;
    btn.addEventListener("click", () => posSetCategory(cat.id));
    container.appendChild(btn);
  }
}

function renderProducts(state, container) {
  container.innerHTML = "";

  if (state.productsLoading) {
    container.textContent = t("common.loading");
    return;
  }

  let products = state.products.filter((p) => p.active);
  if (state.pos.selectedCategoryId !== null) {
    products = products.filter((p) => p.category_id === state.pos.selectedCategoryId);
  }

  if (products.length === 0) {
    container.textContent = t("pos.no_products");
    return;
  }

  for (const p of products) {
    const card = document.createElement("button");
    card.type = "button";
    card.className = "pos-product-card";

    const name = document.createElement("div");
    name.className = "pos-product-name";
    name.textContent = p.name;

    const price = document.createElement("div");
    price.className = "pos-product-price";
    price.textContent = formatPrice(p.price_cents);

    card.append(name, price);
    card.addEventListener("click", () => posAddToCart(p));
    container.appendChild(card);
  }
}

function renderCart(state, container) {
  container.innerHTML = "";

  const header = document.createElement("div");
  header.className = "pos-cart-header";
  header.textContent = t("pos.cart_title");

  container.appendChild(header);

  const list = document.createElement("div");
  list.className = "pos-cart-list";

  if (state.pos.cart.length === 0) {
    const empty = document.createElement("div");
    empty.className = "pos-cart-empty";
    empty.textContent = t("pos.cart_empty");
    list.appendChild(empty);
  } else {
    for (const item of state.pos.cart) {
      const row = document.createElement("div");
      row.className = "pos-cart-row";

      const info = document.createElement("div");
      info.className = "pos-cart-info";

      const name = document.createElement("div");
      name.className = "pos-cart-name";
      name.textContent = item.name;

      const price = document.createElement("div");
      price.className = "pos-cart-price";
      price.textContent = formatPrice(item.priceCents * item.quantity);

      info.append(name, price);

      const controls = document.createElement("div");
      controls.className = "pos-cart-controls";

      const minus = document.createElement("button");
      minus.type = "button";
      minus.textContent = "−";
      minus.addEventListener("click", () => posDecrementFromCart(item.productId));

      const qty = document.createElement("span");
      qty.className = "pos-cart-qty";
      qty.textContent = String(item.quantity);

      const plus = document.createElement("button");
      plus.type = "button";
      plus.textContent = "+";
      plus.addEventListener("click", () => {
        const product = state.products.find((p) => p.id === item.productId);
        if (product) posAddToCart(product);
      });

      const remove = document.createElement("button");
      remove.type = "button";
      remove.className = "pos-cart-remove";
      remove.textContent = "×";
      remove.addEventListener("click", () => posRemoveFromCart(item.productId));

      controls.append(minus, qty, plus, remove);
      row.append(info, controls);
      list.appendChild(row);
    }
  }

  const total = document.createElement("div");
  total.className = "pos-cart-total";
  const totalLabel = document.createElement("span");
  totalLabel.textContent = t("pos.total");
  const totalValue = document.createElement("span");
  totalValue.className = "pos-total-value";
  totalValue.textContent = formatPrice(posCartTotal());
  total.append(totalLabel, totalValue);

  const sendBtn = document.createElement("button");
  sendBtn.type = "button";
  sendBtn.className = "btn btn-primary pos-send-btn";
  sendBtn.textContent = t("pos.send_to_kitchen");
  sendBtn.disabled = state.pos.cart.length === 0 || !state.pos.selectedTableId;
  sendBtn.addEventListener("click", sendToKitchen);

  container.append(list, total, sendBtn);
}

function render(state) {
  const tableSel = document.getElementById("pos-table-selector");
  const catFilter = document.getElementById("pos-category-filter");
  const prodGrid = document.getElementById("pos-products");
  const cartPanel = document.getElementById("pos-cart");

  if (tableSel) renderTableSelector(state, tableSel);
  if (catFilter) renderCategoryFilter(state, catFilter);
  if (prodGrid) renderProducts(state, prodGrid);
  if (cartPanel) renderCart(state, cartPanel);
}

// ---------- Entry point ----------

export function mountPosView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));

  registerViewHook("pos", loadAll);

  render(getState());
  loadAll();
}