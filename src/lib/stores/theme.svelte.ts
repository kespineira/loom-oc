import { browser } from "$app/environment";

export type Theme = "dark" | "light";

const STORAGE_KEY = "loom:theme";

function detectInitial(): Theme {
  if (!browser) return "dark";
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === "dark" || stored === "light") return stored;
  return window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
}

function createTheme() {
  const initial = detectInitial();
  let value = $state<Theme>(initial);

  function apply(next: Theme) {
    if (!browser) return;
    document.documentElement.dataset.theme = next;
  }

  if (browser) apply(initial);

  return {
    get value() {
      return value;
    },
    set(next: Theme) {
      value = next;
      if (browser) {
        localStorage.setItem(STORAGE_KEY, next);
        apply(next);
      }
    },
    toggle() {
      this.set(value === "dark" ? "light" : "dark");
    },
  };
}

export const theme = createTheme();
