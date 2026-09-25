// Thin wrapper around Tauri's event system.
// Backend emits events; the frontend subscribes once and dispatches
// to any number of local handlers.

const { listen } = window.__TAURI__.event;

const handlers = {
  ordersChanged: new Set(),
};

let started = false;

/**
 * Start listening to backend events. Safe to call multiple times.
 * Called once from main.js during app bootstrap.
 */
export async function startEventBridge() {
  if (started) return;
  started = true;

  await listen("orders:changed", () => {
    handlers.ordersChanged.forEach((fn) => {
      try {
        fn();
      } catch (error) {
        console.error("orders:changed handler failed:", error);
      }
    });
  });
}

/**
 * Subscribe to "orders changed" events.
 * Returns an unsubscribe function.
 */
export function onOrdersChanged(handler) {
  handlers.ordersChanged.add(handler);
  return () => handlers.ordersChanged.delete(handler);
}