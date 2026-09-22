// Global application state with a minimal pub/sub mechanism.
// Any part of the UI can subscribe to state changes and react to them.

const listeners = new Set();

let state = {
  tables: [],
  categories: [],
  products: [],
  loading: false,
  productsLoading: false,
  error: null,
  locale: "en",
  currentView: "tables",
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