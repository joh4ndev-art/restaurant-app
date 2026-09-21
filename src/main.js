const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", async () => {
  try {
    const before = await invoke("list_tables");
    console.log("Tables before:", before);

    const created = await invoke("create_table", {
      input: { name: "Mesa 1", zone: "Terraza", capacity: 4 },
    });
    console.log("Created:", created);

    const after = await invoke("list_tables");
    console.log("Tables after:", after);
  } catch (e) {
    console.error("IPC error:", e);
  }
});

import { mountTablesView } from "./js/views/tables.js";

window.addEventListener("DOMContentLoaded", () => {
  mountTablesView();
});