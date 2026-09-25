import { t } from "../i18n/index.js";
import { usersApi } from "../api/ipc.js";
import { setSession } from "../auth.js";

let container = null;

function render() {
  container.innerHTML = "";
  container.classList.remove("hidden");

  const card = document.createElement("div");
  card.className = "login-card";

  const title = document.createElement("h1");
  title.className = "login-title";
  title.textContent = t("login.title");

  const subtitle = document.createElement("p");
  subtitle.className = "login-subtitle";
  subtitle.textContent = t("login.subtitle");

  const form = document.createElement("form");
  form.className = "login-form";

  // Username
  const usernameField = document.createElement("label");
  usernameField.className = "modal-field";

  const usernameLabel = document.createElement("span");
  usernameLabel.textContent = t("login.username");

  const usernameInput = document.createElement("input");
  usernameInput.type = "text";
  usernameInput.autocomplete = "off";
  usernameInput.autofocus = true;

  usernameField.append(usernameLabel, usernameInput);

  // PIN
  const pinField = document.createElement("label");
  pinField.className = "modal-field";

  const pinLabel = document.createElement("span");
  pinLabel.textContent = t("login.pin");

  const pinInput = document.createElement("input");
  pinInput.type = "password";
  pinInput.inputMode = "numeric";
  pinInput.autocomplete = "off";

  pinField.append(pinLabel, pinInput);

  // Error area
  const error = document.createElement("div");
  error.className = "login-error";
  error.style.display = "none";

  // Submit
  const submit = document.createElement("button");
  submit.type = "submit";
  submit.className = "btn btn-primary login-submit";
  submit.textContent = t("login.submit");

  form.append(usernameField, pinField, error, submit);
  card.append(title, subtitle, form);
  container.appendChild(card);

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    error.style.display = "none";

    const res = await usersApi.login({
      username: usernameInput.value.trim(),
      pin: pinInput.value,
    });

    if (!res.ok) {
      error.textContent = t("login.invalid");
      error.style.display = "block";
      pinInput.value = "";
      pinInput.focus();
      return;
    }

    setSession(res.data);

    // Let main.js update the overlay + navigation + router.
    window.__updateLoginVisibility?.();
  });

  usernameInput.focus();
}

export function showLogin() {
  container = document.getElementById("login-overlay");
  if (!container) return;
  render();
}

export function hideLogin() {
  const el = document.getElementById("login-overlay");
  if (el) el.classList.add("hidden");
}