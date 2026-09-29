/**
 * Open a modal form and return its result as a Promise.
 * Resolves with an object of { fieldName: value } on submit,
 * or `null` if the user cancels.
 */
export function openFormModal({
  title,
  fields,
  submitLabel = "Save",
  cancelLabel = "Cancel",
}) {
  return new Promise((resolve) => {
    const backdrop = document.createElement("div");
    backdrop.className = "modal-backdrop";

    const dialog = document.createElement("div");
    dialog.className = "modal-dialog";

    const heading = document.createElement("h3");
    heading.className = "modal-title";
    heading.textContent = title;

    const form = document.createElement("form");
    form.className = "modal-form";

    const inputs = {};

    for (const field of fields) {
      const wrapper = document.createElement("label");
      wrapper.className = "modal-field";

      const label = document.createElement("span");
      label.textContent = field.label;
      wrapper.appendChild(label);

      let input;

      if (field.type === "select") {
        input = document.createElement("select");
        for (const opt of field.options ?? []) {
          const option = document.createElement("option");
          option.value = opt.value;
          option.textContent = opt.label;
          if (field.value !== undefined && String(field.value) === opt.value) {
            option.selected = true;
          }
          input.appendChild(option);
        }
      } else if (field.type === "textarea") {
        input = document.createElement("textarea");
        input.rows = 3;
      } else {
        input = document.createElement("input");
        input.type = field.type ?? "text";
        if (field.step) input.step = field.step;
        if (field.min !== undefined) input.min = field.min;
        if (field.placeholder) input.placeholder = field.placeholder;
      }

      input.name = field.name;
      input.required = !!field.required;

      if (field.value !== undefined && field.value !== null && field.type !== "select") {
        input.value = field.value;
      }

      wrapper.appendChild(input);

      // Hint appears below the input.
      if (field.hint) {
        const hint = document.createElement("small");
        hint.className = "modal-field-hint";
        hint.textContent = field.hint;
        wrapper.appendChild(hint);
      }

      inputs[field.name] = input;
      form.appendChild(wrapper);
    }

    const actions = document.createElement("div");
    actions.className = "modal-actions";

    const cancelBtn = document.createElement("button");
    cancelBtn.type = "button";
    cancelBtn.className = "btn";
    cancelBtn.textContent = cancelLabel;
    cancelBtn.addEventListener("click", () => close(null));

    const submitBtn = document.createElement("button");
    submitBtn.type = "submit";
    submitBtn.className = "btn btn-primary";
    submitBtn.textContent = submitLabel;

    actions.append(cancelBtn, submitBtn);
    form.appendChild(actions);

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      const data = {};
      for (const [name, input] of Object.entries(inputs)) {
        data[name] = input.value;
      }
      close(data);
    });

    dialog.append(heading, form);
    backdrop.appendChild(dialog);
    document.body.appendChild(backdrop);

    function close(result) {
      backdrop.remove();
      document.removeEventListener("keydown", onKey);
      resolve(result);
    }

    function onKey(event) {
      if (event.key === "Escape") close(null);
    }
    document.addEventListener("keydown", onKey);

    // Focus and select the first field automatically.
    const firstInput = Object.values(inputs)[0];
    if (firstInput) {
      firstInput.focus();
      if (typeof firstInput.select === "function") {
        firstInput.select();
      }
    }
  });
}