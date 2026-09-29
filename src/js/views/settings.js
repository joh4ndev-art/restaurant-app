import { t, getLocale, getSupportedLocales, setLocale } from "../i18n/index.js";
import { setState, getState } from "../state/store.js";
import { isAdmin, currentRole } from "../auth.js";
import { openFormModal } from "../components/modal.js";
import { usersApi, printingApi, backupsApi } from "../api/ipc.js";

const LOCALE_LABELS = { en: "English", es: "Español" };

// ---------- Language ----------

function renderLanguageSection() {
  const item = document.createElement("div");
  item.className = "setting-item";

  const info = document.createElement("div");

  const label = document.createElement("div");
  label.className = "setting-label";
  label.textContent = t("settings.language.label");

  const desc = document.createElement("div");
  desc.className = "setting-desc";
  desc.textContent = t("settings.language.description");

  info.append(label, desc);

  const select = document.createElement("select");
  select.className = "setting-select";
  for (const loc of getSupportedLocales()) {
    const opt = document.createElement("option");
    opt.value = loc;
    opt.textContent = LOCALE_LABELS[loc] ?? loc;
    if (loc === getLocale()) opt.selected = true;
    select.appendChild(opt);
  }

  select.addEventListener("change", async () => {
    await setLocale(select.value);
    setState({ locale: select.value });
    renderRoot();
  });

  item.append(info, select);
  return item;
}

// ---------- Restaurant info ----------

async function loadRestaurant() {
  const res = await printingApi.getRestaurantInfo();
  if (!res.ok) return;
  setState({ restaurantInfo: res.data });
  renderRoot();
}

async function editRestaurant() {
  const state = getState();
  const current = state.restaurantInfo ?? { name: "Restaurant", address: null, phone: null, footer_message: null };

  const data = await openFormModal({
    title: t("settings.restaurant.form.title"),
    fields: [
      { name: "name", label: t("settings.restaurant.form.name"), type: "text", required: true, value: current.name },
      { name: "address", label: t("settings.restaurant.form.address"), type: "text", value: current.address ?? "" },
      { name: "phone", label: t("settings.restaurant.form.phone"), type: "text", value: current.phone ?? "" },
      { name: "footer_message", label: t("settings.restaurant.form.footer"), type: "textarea", value: current.footer_message ?? "" },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await printingApi.saveRestaurantInfo({
    name: data.name,
    address: data.address || null,
    phone: data.phone || null,
    footer_message: data.footer_message || null,
  });
  if (!res.ok) {
    alert(t("settings.restaurant.errors.save_failed", { message: res.error.message }));
    return;
  }
  await loadRestaurant();
}

function renderRestaurantSection() {
  const state = getState();
  const info = state.restaurantInfo;

  const item = document.createElement("div");
  item.className = "setting-item";

  const infoBlock = document.createElement("div");

  const label = document.createElement("div");
  label.className = "setting-label";
  label.textContent = t("settings.restaurant.label");

  const desc = document.createElement("div");
  desc.className = "setting-desc";
  desc.textContent = info
    ? `${info.name}${info.address ? " · " + info.address : ""}`
    : t("settings.restaurant.empty");

  infoBlock.append(label, desc);

  const editBtn = document.createElement("button");
  editBtn.type = "button";
  editBtn.className = "btn";
  editBtn.textContent = t("settings.restaurant.edit");
  editBtn.addEventListener("click", editRestaurant);

  item.append(infoBlock, editBtn);
  return item;
}

// ---------- Receipt folder ----------

async function loadReceiptFolder() {
  const res = await printingApi.getReceiptFolder();
  if (!res.ok) return;
  setState({ receiptFolder: res.data });
  renderRoot();
}

async function changeReceiptFolder() {
  const state = getState();
  const data = await openFormModal({
    title: t("settings.printing.folder_form_title"),
    fields: [
      {
        name: "path",
        label: t("settings.printing.folder_path"),
        type: "text",
        required: true,
        value: state.receiptFolder ?? "",
      },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await printingApi.setReceiptFolder(data.path.trim());
  if (!res.ok) {
    alert(t("settings.printing.errors.save_failed", { message: res.error.message }));
    return;
  }
  await loadReceiptFolder();

  fields: [
    {
      name: "path",
      label: t("settings.printing.folder_path"),
      type: "text",
      required: true,
      value: state.receiptFolder ?? "",
      placeholder: "/home/user/Documents/receipts",
      hint: t("settings.printing.folder_hint"),
    },
  ]
}

async function openReceiptFolder() {
  const state = getState();
  if (!state.receiptFolder) return;
  await printingApi.openFile(state.receiptFolder);
}

function renderPrintingSection() {
  const state = getState();

  const item = document.createElement("div");
  item.className = "setting-item";

  const info = document.createElement("div");

  const label = document.createElement("div");
  label.className = "setting-label";
  label.textContent = t("settings.printing.label");

  const desc = document.createElement("div");
  desc.className = "setting-desc";
  desc.textContent = state.receiptFolder || t("common.loading");

  info.append(label, desc);

  const buttons = document.createElement("div");
  buttons.className = "setting-actions";

  const openBtn = document.createElement("button");
  openBtn.type = "button";
  openBtn.className = "btn";
  openBtn.textContent = t("settings.printing.open_folder");
  openBtn.addEventListener("click", openReceiptFolder);

  const changeBtn = document.createElement("button");
  changeBtn.type = "button";
  changeBtn.className = "btn";
  changeBtn.textContent = t("settings.printing.change_folder");
  changeBtn.addEventListener("click", changeReceiptFolder);

  buttons.append(openBtn, changeBtn);

  item.append(info, buttons);
  return item;
}

// ---------- Backup ----------

async function loadBackups() {
  const [listRes, folderRes, autoRes] = await Promise.all([
    backupsApi.list(),
    backupsApi.getFolder(),
    backupsApi.getAuto(),
  ]);

  if (!listRes.ok) {
    alert(t("settings.backup.errors.load_failed", { message: listRes.error.message }));
    return;
  }
  if (!folderRes.ok) {
    alert(t("settings.backup.errors.load_failed", { message: folderRes.error.message }));
    return;
  }
  if (!autoRes.ok) {
    alert(t("settings.backup.errors.load_failed", { message: autoRes.error.message }));
    return;
  }

  setState({
    backups: listRes.data,
    backupFolder: folderRes.data,
    autoBackup: autoRes.data,
  });
  renderRoot();
}

async function createBackupNow() {
  const res = await backupsApi.create();
  if (!res.ok) {
    alert(t("settings.backup.errors.create_failed", { message: res.error.message }));
    return;
  }
  alert(t("settings.backup.created", { filename: res.data.filename }));
  await loadBackups();
}

async function restoreBackup(backup) {
  const warning = t("settings.backup.confirm_restore", { filename: backup.filename });
  if (!confirm(warning)) return;

  const res = await backupsApi.restore(backup.filename);
  if (!res.ok) {
    alert(t("settings.backup.errors.restore_failed", { message: res.error.message }));
    return;
  }

  alert(t("settings.backup.restored"));
  // Full reload: the entire DB state changed under our feet.
  window.location.reload();
}

async function deleteBackup(backup) {
  if (!confirm(t("settings.backup.confirm_delete", { filename: backup.filename }))) return;

  const res = await backupsApi.remove(backup.filename);
  if (!res.ok) {
    alert(t("settings.backup.errors.delete_failed", { message: res.error.message }));
    return;
  }
  await loadBackups();
}

async function openBackupFolder() {
  const state = getState();
  if (!state.backupFolder) return;
  await printingApi.openFile(state.backupFolder);
}

async function changeBackupFolder() {
  const state = getState();
  const data = await openFormModal({
    title: t("settings.backup.folder_form_title"),
    fields: [
      {
        name: "path",
        label: t("settings.backup.folder_path"),
        type: "text",
        required: true,
        value: state.backupFolder ?? "",
      },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await backupsApi.setFolder(data.path.trim());
  if (!res.ok) {
    alert(t("settings.backup.errors.save_failed", { message: res.error.message }));
    return;
  }
  await loadBackups();
  fields: [
    {
      name: "path",
      label: t("settings.backup.folder_path"),
      type: "text",
      required: true,
      value: state.backupFolder ?? "",
      placeholder: "/home/user/backups",
      hint: t("settings.backup.folder_hint"),
    },
  ]
}

async function toggleAutoBackup(enabled) {
  const res = await backupsApi.setAuto(enabled);
  if (!res.ok) {
    alert(t("settings.backup.errors.save_failed", { message: res.error.message }));
    return;
  }
  setState({ autoBackup: enabled });
  renderRoot();
}

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

function renderBackupSection() {
  const state = getState();

  const wrapper = document.createElement("div");
  wrapper.className = "settings-backup";

  // Header row with toggle
  const headerRow = document.createElement("div");
  headerRow.className = "setting-item";

  const headerInfo = document.createElement("div");

  const autoLabel = document.createElement("div");
  autoLabel.className = "setting-label";
  autoLabel.textContent = t("settings.backup.auto_label");

  const autoDesc = document.createElement("div");
  autoDesc.className = "setting-desc";
  autoDesc.textContent = t("settings.backup.auto_description");

  headerInfo.append(autoLabel, autoDesc);

  const toggle = document.createElement("input");
  toggle.type = "checkbox";
  toggle.className = "setting-toggle";
  toggle.checked = state.autoBackup;
  toggle.addEventListener("change", () => toggleAutoBackup(toggle.checked));

  headerRow.append(headerInfo, toggle);
  wrapper.appendChild(headerRow);

  // Folder row
  const folderRow = document.createElement("div");
  folderRow.className = "setting-item";

  const folderInfo = document.createElement("div");

  const folderLabel = document.createElement("div");
  folderLabel.className = "setting-label";
  folderLabel.textContent = t("settings.backup.folder_label");

  const folderDesc = document.createElement("div");
  folderDesc.className = "setting-desc";
  folderDesc.textContent = state.backupFolder || t("common.loading");

  folderInfo.append(folderLabel, folderDesc);

  const folderActions = document.createElement("div");
  folderActions.className = "setting-actions";

  const openBtn = document.createElement("button");
  openBtn.type = "button";
  openBtn.className = "btn";
  openBtn.textContent = t("settings.backup.open_folder");
  openBtn.addEventListener("click", openBackupFolder);

  const changeBtn = document.createElement("button");
  changeBtn.type = "button";
  changeBtn.className = "btn";
  changeBtn.textContent = t("settings.backup.change_folder");
  changeBtn.addEventListener("click", changeBackupFolder);

  folderActions.append(openBtn, changeBtn);
  folderRow.append(folderInfo, folderActions);
  wrapper.appendChild(folderRow);

  // Actions row
  const actionsRow = document.createElement("div");
  actionsRow.className = "settings-backup-actions";

  const createBtn = document.createElement("button");
  createBtn.type = "button";
  createBtn.className = "btn btn-primary";
  createBtn.textContent = t("settings.backup.create_now");
  createBtn.addEventListener("click", createBackupNow);

  actionsRow.appendChild(createBtn);
  wrapper.appendChild(actionsRow);

  // Backups list
  if (state.backups.length === 0) {
    const empty = document.createElement("div");
    empty.className = "settings-backup-empty";
    empty.textContent = t("settings.backup.empty");
    wrapper.appendChild(empty);
    return wrapper;
  }

  const list = document.createElement("div");
  list.className = "settings-backup-list";

  for (const b of state.backups) {
    const row = document.createElement("div");
    row.className = "settings-backup-row";

    const info = document.createElement("div");
    info.className = "settings-backup-info";

    const name = document.createElement("div");
    name.className = "settings-backup-name";
    name.textContent = b.filename;

    const meta = document.createElement("div");
    meta.className = "settings-backup-meta";
    meta.textContent = `${b.created_at} · ${formatBytes(b.size_bytes)}`;

    info.append(name, meta);

    const actions = document.createElement("div");
    actions.className = "settings-backup-row-actions";

    const restoreBtn = document.createElement("button");
    restoreBtn.type = "button";
    restoreBtn.className = "btn";
    restoreBtn.textContent = t("settings.backup.restore");
    restoreBtn.addEventListener("click", () => restoreBackup(b));

    const delBtn = document.createElement("button");
    delBtn.type = "button";
    delBtn.className = "btn btn-danger";
    delBtn.textContent = t("settings.backup.delete");
    delBtn.addEventListener("click", () => deleteBackup(b));

    actions.append(restoreBtn, delBtn);
    row.append(info, actions);
    list.appendChild(row);
  }

  wrapper.appendChild(list);
  return wrapper;
}

// ---------- Users ----------

async function loadUsers() {
  const res = await usersApi.list();
  if (!res.ok) {
    alert(t("settings.users.errors.load_failed", { message: res.error.message }));
    return;
  }
  setState({ users: res.data });
  renderRoot();
}

async function addUser() {
  const data = await openFormModal({
    title: t("settings.users.form.title_create"),
    fields: [
      { name: "username", label: t("settings.users.form.username"), type: "text", required: true },
      { name: "pin", label: t("settings.users.form.pin"), type: "text", required: true },
      {
        name: "role",
        label: t("settings.users.form.role"),
        type: "select",
        options: [
          { value: "admin",   label: t("settings.users.roles.admin") },
          { value: "waiter",  label: t("settings.users.roles.waiter") },
          { value: "kitchen", label: t("settings.users.roles.kitchen") },
        ],
      },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await usersApi.create({
    username: data.username,
    pin: data.pin,
    role: data.role,
  });
  if (!res.ok) {
    alert(t("settings.users.errors.create_failed", { message: res.error.message }));
    return;
  }
  await loadUsers();
}

async function editUser(user) {
  const data = await openFormModal({
    title: t("settings.users.form.title_edit"),
    fields: [
      { name: "username", label: t("settings.users.form.username"), type: "text", required: true, value: user.username },
      {
        name: "role",
        label: t("settings.users.form.role"),
        type: "select",
        options: [
          { value: "admin",   label: t("settings.users.roles.admin") },
          { value: "waiter",  label: t("settings.users.roles.waiter") },
          { value: "kitchen", label: t("settings.users.roles.kitchen") },
        ],
        value: user.role,
      },
      {
        name: "active",
        label: t("settings.users.form.active"),
        type: "select",
        options: [
          { value: "true",  label: t("common.yes") },
          { value: "false", label: t("common.no") },
        ],
        value: user.active ? "true" : "false",
      },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await usersApi.update({
    id: user.id,
    username: data.username,
    role: data.role,
    active: data.active === "true",
  });
  if (!res.ok) {
    alert(t("settings.users.errors.update_failed", { message: res.error.message }));
    return;
  }
  await loadUsers();
}

async function changePin(user) {
  const data = await openFormModal({
    title: t("settings.users.form.title_change_pin", { username: user.username }),
    fields: [
      { name: "pin", label: t("settings.users.form.new_pin"), type: "text", required: true },
    ],
    submitLabel: t("settings.users.form.save"),
    cancelLabel: t("settings.form.cancel"),
  });
  if (!data) return;

  const res = await usersApi.changePin({ id: user.id, pin: data.pin });
  if (!res.ok) {
    alert(t("settings.users.errors.pin_failed", { message: res.error.message }));
    return;
  }
  alert(t("settings.users.pin_changed"));
}

async function deleteUser(user) {
  if (!confirm(t("settings.users.confirm_delete", { username: user.username }))) return;

  const res = await usersApi.remove(user.id);
  if (!res.ok) {
    alert(t("settings.users.errors.delete_failed", { message: res.error.message }));
    return;
  }
  await loadUsers();
}

function renderUsersSection() {
  const state = getState();

  const wrapper = document.createElement("div");
  wrapper.className = "settings-users";

  const header = document.createElement("div");
  header.className = "settings-users-header";

  const title = document.createElement("div");
  title.className = "setting-label";
  title.textContent = t("settings.users.title");

  const addBtn = document.createElement("button");
  addBtn.type = "button";
  addBtn.className = "btn btn-primary";
  addBtn.textContent = t("settings.users.add");
  addBtn.addEventListener("click", addUser);

  header.append(title, addBtn);
  wrapper.appendChild(header);

  if (state.users.length === 0) {
    const empty = document.createElement("div");
    empty.className = "settings-users-empty";
    empty.textContent = t("settings.users.empty");
    wrapper.appendChild(empty);
    return wrapper;
  }

  const table = document.createElement("div");
  table.className = "settings-users-table";

  for (const u of state.users) {
    const row = document.createElement("div");
    row.className = "settings-users-row";

    const info = document.createElement("div");
    info.className = "settings-users-info";

    const name = document.createElement("div");
    name.className = "settings-users-name";
    name.textContent = u.username;
    if (!u.active) {
      const badge = document.createElement("span");
      badge.className = "settings-users-badge";
      badge.textContent = t("settings.users.inactive");
      name.appendChild(badge);
    }

    const role = document.createElement("div");
    role.className = "settings-users-role";
    role.textContent = t(`settings.users.roles.${u.role}`);

    info.append(name, role);

    const actions = document.createElement("div");
    actions.className = "settings-users-actions";

    const editBtn = document.createElement("button");
    editBtn.type = "button";
    editBtn.className = "btn";
    editBtn.textContent = t("settings.users.edit");
    editBtn.addEventListener("click", () => editUser(u));

    const pinBtn = document.createElement("button");
    pinBtn.type = "button";
    pinBtn.className = "btn";
    pinBtn.textContent = t("settings.users.change_pin");
    pinBtn.addEventListener("click", () => changePin(u));

    const delBtn = document.createElement("button");
    delBtn.type = "button";
    delBtn.className = "btn btn-danger";
    delBtn.textContent = t("settings.users.delete");
    delBtn.addEventListener("click", () => deleteUser(u));

    actions.append(editBtn, pinBtn, delBtn);
    row.append(info, actions);
    table.appendChild(row);
  }

  wrapper.appendChild(table);
  return wrapper;
}

// ---------- Root ----------

function renderRoot() {
  const root = document.getElementById("view-settings");
  if (!root) return;

  root.innerHTML = "";

  const title = document.createElement("h2");
  title.textContent = t("settings.title");
  title.style.marginTop = "0";

  const list = document.createElement("div");
  list.className = "settings-list";
  list.appendChild(renderLanguageSection());
  list.appendChild(renderRestaurantSection());
  list.appendChild(renderPrintingSection());

  root.append(title, list);

  if (isAdmin()) {
    const backupHeader = document.createElement("h2");
    backupHeader.textContent = t("settings.backup.title");
    backupHeader.style.marginTop = "2rem";
    root.append(backupHeader, renderBackupSection());

    const usersHeader = document.createElement("h2");
    usersHeader.textContent = t("settings.users.title");
    usersHeader.style.marginTop = "2rem";
    root.append(usersHeader, renderUsersSection());
  }
}

// ---------- Entry point ----------

export function mountSettingsView() {
  renderRoot();
  loadRestaurant();
  loadReceiptFolder();
  if (isAdmin()) {
    loadBackups();
    loadUsers();
  }
}

export function reloadSettingsView() {
  renderRoot();
  loadRestaurant();
  loadReceiptFolder();
  if (isAdmin()) {
    loadBackups();
    loadUsers();
  }
}