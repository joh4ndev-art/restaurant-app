import { subscribe, setState, getState } from "../state/store.js";
import { t, onLocaleChange } from "../i18n/index.js";
import { categoriesApi, productsApi } from "../api/ipc.js";
import { openFormModal } from "../components/modal.js";
import { registerViewHook } from "../router.js";

// ---------- Helpers ----------

function formatPrice(cents) {
  return `$${(cents / 100).toFixed(2)}`;
}

function parsePriceToCents(str) {
  const value = parseFloat(str);
  if (!Number.isFinite(value) || value <= 0) return null;
  return Math.round(value * 100);
}

// ---------- Data loading ----------

async function loadAll() {
  setState({ productsLoading: true, error: null });

  const [cats, prods] = await Promise.all([
    categoriesApi.list(),
    productsApi.list(),
  ]);

  if (!cats.ok || !prods.ok) {
    setState({
      productsLoading: false,
      error: cats.error?.message ?? prods.error?.message ?? "unknown error",
    });
    return;
  }

  setState({
    categories: cats.data,
    products: prods.data,
    productsLoading: false,
  });
}

// ---------- Category operations ----------

async function addCategory() {
  const data = await openFormModal({
    title: t("products.category_form.title"),
    fields: [
      {
        name: "name",
        label: t("products.category_form.name"),
        type: "text",
        required: true,
      },
    ],
    submitLabel: t("products.category_form.save"),
    cancelLabel: t("products.form.cancel"),
  });
  if (!data) return;

  const res = await categoriesApi.create({ name: data.name });
  if (!res.ok) {
    alert(t("products.errors.category_create_failed", { message: res.error.message }));
    return;
  }
  await loadAll();
}

async function deleteCategory(category) {
  if (!confirm(t("products.confirm_delete_category", { name: category.name }))) return;

  const res = await categoriesApi.remove(category.id);
  if (!res.ok) {
    alert(t("products.errors.category_delete_failed", { message: res.error.message }));
    return;
  }
  await loadAll();
}

// ---------- Product operations ----------

async function addProduct() {
  const state = getState();
  if (state.categories.length === 0) {
    alert(t("products.no_categories"));
    return;
  }

  const data = await openFormModal({
    title: t("products.form.title_create"),
    fields: [
      {
        name: "name",
        label: t("products.form.name"),
        type: "text",
        required: true,
      },
      {
        name: "category_id",
        label: t("products.form.category"),
        type: "select",
        options: state.categories.map((c) => ({
          value: String(c.id),
          label: c.name,
        })),
      },
      {
        name: "price",
        label: t("products.form.price"),
        type: "number",
        step: "0.01",
        min: "0.01",
        required: true,
      },
      {
        name: "description",
        label: t("products.form.description"),
        type: "textarea",
      },
    ],
    submitLabel: t("products.form.save"),
    cancelLabel: t("products.form.cancel"),
  });
  if (!data) return;

  const cents = parsePriceToCents(data.price);
  if (cents === null) {
    alert(t("products.errors.invalid_price"));
    return;
  }

  const res = await productsApi.create({
    category_id: parseInt(data.category_id, 10),
    name: data.name,
    description: data.description || null,
    price_cents: cents,
  });
  if (!res.ok) {
    alert(t("products.errors.create_failed", { message: res.error.message }));
    return;
  }
  await loadAll();
}

async function deleteProduct(product) {
  if (!confirm(t("products.confirm_delete_product", { name: product.name }))) return;

  const res = await productsApi.remove(product.id);
  if (!res.ok) {
    alert(t("products.errors.delete_failed", { message: res.error.message }));
    return;
  }
  await loadAll();
}

// ---------- Rendering ----------

function productCard(product) {
  const card = document.createElement("div");
  card.className = "product-card";

  const name = document.createElement("div");
  name.className = "product-name";
  name.textContent = product.name;

  const price = document.createElement("div");
  price.className = "product-price";
  price.textContent = formatPrice(product.price_cents);

  const del = document.createElement("button");
  del.type = "button";
  del.className = "product-delete";
  del.textContent = "×";
  del.title = t("products.delete_product");
  del.addEventListener("click", () => deleteProduct(product));

  card.append(name, price, del);
  return card;
}

function render(state) {
  const root = document.getElementById("products-container");
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

  if (state.categories.length === 0) {
    root.textContent = t("products.empty");
    return;
  }

  const grouped = state.products.reduce((acc, product) => {
    (acc[product.category_id] ??= []).push(product);
    return acc;
  }, {});

  for (const category of state.categories) {
    const section = document.createElement("section");
    section.className = "category-section";

    const header = document.createElement("div");
    header.className = "category-header";

    const title = document.createElement("h3");
    title.textContent = category.name;

    const delBtn = document.createElement("button");
    delBtn.type = "button";
    delBtn.className = "category-delete";
    delBtn.textContent = "×";
    delBtn.title = t("products.delete_category");
    delBtn.addEventListener("click", () => deleteCategory(category));

    header.append(title, delBtn);

    const grid = document.createElement("div");
    grid.className = "products-grid";

    const items = grouped[category.id] ?? [];
    if (items.length === 0) {
      const empty = document.createElement("div");
      empty.className = "products-empty";
      empty.textContent = t("products.category_empty");
      grid.appendChild(empty);
    } else {
      for (const p of items) {
        grid.appendChild(productCard(p));
      }
    }

    section.append(header, grid);
    root.appendChild(section);
  }
}

// ---------- Entry point ----------

export function mountProductsView() {
  subscribe(render);
  onLocaleChange(() => render(getState()));

  document.getElementById("add-product-btn")?.addEventListener("click", addProduct);
  document.getElementById("add-category-btn")?.addEventListener("click", addCategory);

  registerViewHook("products", loadAll);

  render(getState());
  loadAll();
}