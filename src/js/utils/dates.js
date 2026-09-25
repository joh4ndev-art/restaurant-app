/**
 * Return a "YYYY-MM-DD" string for the given Date object,
 * using the user's local timezone (not UTC).
 */
export function toISODate(date) {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

export function today() {
  return toISODate(new Date());
}

export function daysAgo(n) {
  const d = new Date();
  d.setDate(d.getDate() - n);
  return toISODate(d);
}

export function startOfMonth() {
  const d = new Date();
  d.setDate(1);
  return toISODate(d);
}

export function formatDisplayDate(isoDate) {
  // isoDate: "YYYY-MM-DD". Show as "DD/MM/YYYY" in Spanish-friendly format.
  const [y, m, d] = isoDate.split("-");
  return `${d}/${m}/${y}`;
}