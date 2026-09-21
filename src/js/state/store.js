// Global application state with a minimal pub/sub mechanism.
// Any part of the UI can subscribe to state changes and react to them.

const listeners = new Set();

let state = {
  tables: [],
  loading: false,
  error: null,
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