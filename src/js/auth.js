import { setState, getState } from "./state/store.js";
import { usersApi } from "./api/ipc.js";

/**
 * Role → allowed views mapping.
 * Any view not listed here is inaccessible for that role.
 */
const ROLE_VIEWS = {
  admin:   ["tables", "pos", "orders", "kitchen", "products", "reports", "settings"],
  waiter:  ["tables", "pos", "orders", "kitchen"],
  kitchen: ["kitchen"],
};

/// Effective role of the current session.
/// In free mode (no users yet), the user is treated as admin.
export function currentRole() {
  const { session } = getState();
  if (session.isFreeMode) return "admin";
  return session.user?.role ?? null;
}

export function isAdmin() {
  return currentRole() === "admin";
}

export function canAccess(view) {
  const role = currentRole();
  if (!role) return false;
  return ROLE_VIEWS[role]?.includes(view) ?? false;
}

/// List of views accessible to the current session, in nav order.
export function allowedViews() {
  const role = currentRole();
  if (!role) return [];
  return ROLE_VIEWS[role] ?? [];
}

/// Set the current session after a successful login.
export function setSession(user) {
  setState({ session: { user, isFreeMode: false } });
}

/// Set free mode (no users exist yet).
export function setFreeMode() {
  setState({ session: { user: null, isFreeMode: true } });
}

/// Clear the session (logout).
export function clearSession() {
  setState({ session: { user: null, isFreeMode: false } });
}

/// On startup: query the backend to know if users exist.
export async function detectSessionMode() {
  const res = await usersApi.hasUsers();
  if (!res.ok) {
    console.error("hasUsers failed:", res.error);
    return;
  }
  if (res.data) {
    // Users exist; login required. Session stays empty until login.
    setState({ session: { user: null, isFreeMode: false } });
  } else {
    setFreeMode();
  }
}