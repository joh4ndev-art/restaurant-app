import { subscribe, setState, getState } from "../state/store.js";
import { tablesApi } from "../api/ipc.js";

// Status cycle when clicking a table.
const STATUS_CYCLE = {
  free: "occupied",
  occupied: "cleaning",
  cleaning: "free",
  reserved: "occupied",
};

const STATUS_LABEL = {
  free: "Free",
  occupied: "Occupied",
  reserved: "Reserved",
  cleaning: "Cleaning",
};

// ---------- Data loading ----------

async function loadTables() {
  setState({ loading: true, error: null });
  const result = await tablesApi.list();

  if (!result.ok) {
    setState({ loading: false, error: result.error.message });
    return;
  }

  setState({ tables: result.data, loading: false });
}

async function createTable() {
  const name = prompt("Table name:");
  if (!name) return;

  const zone = prompt("Zone (optional):") || null;
  const capacityStr = prompt("Capacity:", "4");
  const capacity = parseInt(capacityStr, 10);

  if (!Number.isInteger(capacity) || capacity <= 0) {
    alert("Capacity must be a positive number.");
    return;
  }

  const result = await tablesApi.create({ name, zone, capacity });
  if (!result.ok) {
    alert(`Error: ${result.error.message}`);
    return;
  }

  await loadTables();
}

async function cycleStatus(table) {
  const next = STATUS_CYCLE[table.status] ?? "free";
  const result = await tablesApi.updateStatus(table.id, next);

  if (!result.ok) {
    alert(`Error: ${result.error.message}`);
    return;
  }

  await loadTables();
}

async function deleteTable(table) {
  if (!confirm(`Delete "${table.name}"?`)) return;

  const result = await tablesApi.remove(table.id);
  if (!result.ok) {
    alert(`Error: ${result.error.message}`);
    return;
  }

  await loadTables();
}

// ---------- Rendering ----------

function tableCard(table) {
  const card = document.createElement("button");
  card.type = "button";
  card.className = `table-card status-${table.status}`;

  const name = document.createElement("span");
  name.className = "table-name";
  name.textContent = table.name;

  const meta = document.createElement("span");
  meta.className = "table-meta";
  const zone = table.zone ? ` · ${table.zone}` : "";
  meta.textContent = `${STATUS_LABEL[table.status]}${zone} · ${table.capacity}p`;

  const del = document.createElement("span");
  del.className = "table-delete";
  del.textContent = "×";
  del.title = "Delete table";
  del.addEventListener("click", (event) => {
    event.stopPropagation();
    deleteTable(table);
  });

  card.append(name, meta, del);
  card.addEventListener("click", () => cycleStatus(table));
  return card;
}

function render(state) {
  const root = document.getElementById("tables-grid");
  if (!root) return;

  root.innerHTML = "";

  if (state.loading) {
    root.textContent = "Loading...";
    return;
  }

  if (state.error) {
    root.textContent = `Error: ${state.error}`;
    return;
  }

  if (state.tables.length === 0) {
    root.textContent = "No tables yet. Click 'Add table' to create one.";
    return;
  }

  for (const table of state.tables) {
    root.appendChild(tableCard(table));
  }
}

// ---------- Entry point ----------

export function mountTablesView() {
  subscribe(render);

  document
    .getElementById("add-table-btn")
    ?.addEventListener("click", createTable);

  render(getState());
  loadTables();
}