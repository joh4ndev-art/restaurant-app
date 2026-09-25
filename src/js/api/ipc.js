// Single entry point between the frontend and the Rust backend.
// Every IPC call goes through `call()`, which normalizes the response.

const { invoke } = window.__TAURI__.core;

async function call(command, args = {}) {
  try {
    const data = await invoke(command, args);
    return { ok: true, data };
  } catch (error) {
    // Errors coming from Rust are already shaped as { code, message }.
    return {
      ok: false,
      error: {
        code: error?.code ?? "unknown_error",
        message: error?.message ?? String(error),
      },
    };
  }
}

export const tablesApi = {
  list: () => call("list_tables"),
  create: (input) => call("create_table", { input }),
  updateStatus: (id, status) => call("update_table_status", { id, status }),
  remove: (id) => call("delete_table", { id }),
};

export const settingsApi = {
  get: (key) => call("get_setting", { key }),
  set: (key, value) => call("set_setting", { key, value }),
};

export const categoriesApi = {
  list: () => call("list_categories"),
  create: (input) => call("create_category", { input }),
  remove: (id) => call("delete_category", { id }),
};

export const productsApi = {
  list: () => call("list_products"),
  create: (input) => call("create_product", { input }),
  update: (input) => call("update_product", { input }),
  remove: (id) => call("delete_product", { id }),
};

export const ordersApi = {
  send: (input) => call("send_order", { input }),
  listOpen: () => call("list_open_orders"),
  get: (id) => call("get_order", { id }),
  cancel: (id) => call("cancel_order", { id }),
  getActiveForTable: (tableId) => call("get_active_order_for_table", { table_id: tableId }),
  markServed: (id) => call("mark_order_served", { id }),
  listByStatuses: (statuses) => call("list_orders_by_status", { statuses }),
};


export const paymentsApi = {
  pay: (input) => call("pay_order", { input }),
  summary: (orderId) => call("get_payment_summary", { order_id: orderId }),
};


export const reportsApi = {
  generate: (input) => call("generate_report", { input }),
};

export const usersApi = {
  hasUsers: () => call("has_users"),
  login: (input) => call("login", { input }),
  list: () => call("list_users"),
  create: (input) => call("create_user", { input }),
  update: (input) => call("update_user", { input }),
  changePin: (input) => call("change_user_pin", { input }),
  remove: (id) => call("delete_user", { id }),
};