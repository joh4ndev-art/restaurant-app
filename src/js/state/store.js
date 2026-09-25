// Global application state with a minimal pub/sub mechanism.
// Any part of the UI can subscribe to state changes and react to them.

const listeners = new Set();

let state = {
  // existing
  tables: [],
  categories: [],
  products: [],
  loading: false,
  productsLoading: false,
  error: null,
  locale: "en",
  currentView: "tables",

  // POS
  pos: {
    selectedTableId: null,
    selectedCategoryId: null,
    cart: [],
  },

  // Kitchen, Orders, Reports
  orders: [],
  kitchenOrders: [],
  kitchenLoading: false,
  reports: null,
  reportsLoading: false,
  reportsRange: null,

  // NEW: session and users
  session: {
    user: null,         // User | null
    isFreeMode: false,  // true when no users exist
  },
  users: [],
};

export function getState() {
  return state;
}

export function setState(patch) {
  state = { ...state, ...patch };
  listeners.forEach((fn) => fn(state));
}

export function subscribe(fn) {
  listeners.add(fn);
  return () => listeners.delete(fn);
}


// ---------- POS helpers ----------

export function posSetTable(tableId) {
  const state = getState();
  setState({ pos: { ...state.pos, selectedTableId: tableId } });
}

export function posSetCategory(categoryId) {
  const state = getState();
  setState({ pos: { ...state.pos, selectedCategoryId: categoryId } });
}

export function posAddToCart(product) {
  const state = getState();
  const cart = [...state.pos.cart];
  const existing = cart.find((item) => item.productId === product.id);

  if (existing) {
    existing.quantity += 1;
  } else {
    cart.push({
      productId: product.id,
      name: product.name,
      priceCents: product.price_cents,
      quantity: 1,
      notes: null,
    });
  }

  setState({ pos: { ...state.pos, cart } });
}

export function posDecrementFromCart(productId) {
  const state = getState();
  let cart = [...state.pos.cart];
  const existing = cart.find((item) => item.productId === productId);

  if (!existing) return;

  if (existing.quantity > 1) {
    existing.quantity -= 1;
  } else {
    cart = cart.filter((item) => item.productId !== productId);
  }

  setState({ pos: { ...state.pos, cart } });
}

export function posRemoveFromCart(productId) {
  const state = getState();
  const cart = state.pos.cart.filter((item) => item.productId !== productId);
  setState({ pos: { ...state.pos, cart } });
}

export function posClearCart() {
  const state = getState();
  setState({ pos: { ...state.pos, cart: [] } });
}

export function posCartTotal() {
  return getState().pos.cart.reduce(
    (sum, item) => sum + item.priceCents * item.quantity,
    0
  );
}