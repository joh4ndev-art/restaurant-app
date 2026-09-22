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