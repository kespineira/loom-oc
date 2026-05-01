# Design System — Loom

Loom is a visual configuration editor for [OpenCode](https://opencode.ai). This document is the source of truth for its visual and interaction design.

The audience is developers — primarily DevOps and platform engineers — editing dense technical configuration. The design system is built around that.

---

## 1. Principles

1. **Dense but breathable.** Users edit a lot of config at once. Use space efficiently, but leave air between logical groups.
2. **Text before icons.** Icons are used only when they add real recognition (status, severity, scope). No decorative icons next to every label.
3. **Honest state.** When something is unsaved, it shows. When something is destructive, it warns. When something differs from disk, the diff is visible.
4. **Keyboard-first.** Power users live in shortcuts. Every primary action has one, focus is always visible, tab order is intentional.
5. **Boring on purpose.** This is a configuration tool, not a consumer app. No bouncy animations, no gradients, no marketing flourishes.

---

## 2. Tokens

All tokens live in `src/app.css` as CSS custom properties. No external token systems, no Tailwind config beyond mapping these variables.

### 2.1 Color

The palette is neutral-cool with a single warm accent (amber). Dark by default; light theme overrides only the tokens that change.

```css
:root {
  /* Surfaces */
  --bg-0: #0d0f12;          /* window background */
  --bg-1: #14171c;          /* panels, cards */
  --bg-2: #1b1f26;          /* elevated, hover */
  --bg-3: #232830;          /* hover over bg-2, table headers */

  /* Borders */
  --border-subtle:  #232830;
  --border-default: #2d333d;
  --border-strong:  #3a4250;

  /* Text */
  --text-primary:   #e6e8eb;
  --text-secondary: #a0a6b0;
  --text-tertiary:  #6b7280;
  --text-disabled:  #4a5160;

  /* Accent (single, amber) */
  --accent:        #e8a849;
  --accent-hover:  #f0b65f;
  --accent-muted:  #2a2118;
  --accent-text:   #1a1410;

  /* Semantic states */
  --success:       #4ade80;
  --success-muted: #14271c;
  --warning:       #fbbf24;
  --warning-muted: #2a2114;
  --danger:        #f87171;
  --danger-muted:  #2a1818;
  --info:          #60a5fa;
  --info-muted:    #14202e;

  /* Diff viewer */
  --diff-add-bg:        rgba(74, 222, 128, 0.12);
  --diff-add-border:    rgba(74, 222, 128, 0.40);
  --diff-remove-bg:     rgba(248, 113, 113, 0.12);
  --diff-remove-border: rgba(248, 113, 113, 0.40);
}

[data-theme="light"] {
  --bg-0: #fafafa;
  --bg-1: #ffffff;
  --bg-2: #f4f4f5;
  --bg-3: #e8e8eb;

  --border-subtle:  #e8e8eb;
  --border-default: #d4d4d8;
  --border-strong:  #a1a1aa;

  --text-primary:   #18181b;
  --text-secondary: #52525b;
  --text-tertiary:  #71717a;
  --text-disabled:  #a1a1aa;

  --accent:        #b45309;
  --accent-hover:  #92400e;
  --accent-muted:  #fef3c7;
  --accent-text:   #ffffff;
}
```

**Rules**

- Never use raw hex values in components — always reference tokens.
- Every text/background pair listed here meets WCAG AA. If you add a new pair, verify with a contrast checker.
- The accent is the only warm color. Do not introduce a second accent.

### 2.2 Typography

Three mental categories:

- **Sans (Inter)** — prose, labels, buttons, navigation.
- **Mono (JetBrains Mono)** — any value the user would write inside a `.json` file. Model names (`anthropic/claude-sonnet-4-5`), paths, JSON keys, commands, IDs.
- **Tabular numerals** — for tables of numbers (timeouts, rate limits): `font-variant-numeric: tabular-nums`.

```css
:root {
  --font-sans: "Inter", -apple-system, "Segoe UI", system-ui, sans-serif;
  --font-mono: "JetBrains Mono", "SF Mono", "Cascadia Code", Menlo, monospace;

  --text-xs:   11px;
  --text-sm:   12px;
  --text-base: 13px;   /* body. Yes, 13. The app is dense. */
  --text-md:   14px;
  --text-lg:   16px;
  --text-xl:   20px;
  --text-2xl:  24px;

  --leading-tight:   1.30;
  --leading-normal:  1.50;
  --leading-relaxed: 1.65;
}
```

**Hierarchy**

| Level   | Size | Weight | Usage                                              |
| ------- | ---- | ------ | -------------------------------------------------- |
| Display | 24   | 600    | Main app header, once per screen                   |
| H1      | 20   | 600    | Page title (Agents, MCP Servers, Permissions)      |
| H2      | 16   | 600    | Section title within a page                        |
| H3      | 14   | 600    | Subsection, group label                            |
| Body    | 13   | 400    | Default body text, descriptions                    |
| Label   | 12   | 500    | Form labels                                        |
| Caption | 11   | 400    | Helper text, metadata, badges                      |

### 2.3 Spacing

4px base scale. Do not invent values outside this scale.

```css
:root {
  --space-1:  4px;
  --space-2:  8px;
  --space-3:  12px;
  --space-4:  16px;
  --space-5:  24px;
  --space-6:  32px;
  --space-8:  48px;
  --space-10: 64px;
}
```

### 2.4 Radii

```css
:root {
  --radius-sm: 4px;   /* badges, tags */
  --radius-md: 6px;   /* inputs, buttons */
  --radius-lg: 8px;   /* cards, panels */
  --radius-xl: 12px;  /* modals */
}
```

### 2.5 Shadows

Sparingly used. In dark mode, shadows are nearly invisible — use borders for separation instead.

```css
:root {
  --shadow-sm: 0 1px 2px  rgba(0, 0, 0, 0.30);
  --shadow-md: 0 4px 8px  rgba(0, 0, 0, 0.40);
  --shadow-lg: 0 12px 32px rgba(0, 0, 0, 0.50);
}
```

### 2.6 Layout

```css
:root {
  --sidebar-width:    220px;
  --titlebar-height:  36px;
  --statusbar-height: 28px;
}
```

### 2.7 Motion

```css
:root {
  --duration-fast:   100ms;
  --duration-normal: 150ms;
  --duration-slow:   250ms;
  --ease: cubic-bezier(0.4, 0, 0.2, 1);
}
```

**Rules**

- Hover/focus transitions: `--duration-fast`.
- Menus, popovers, toasts: `--duration-normal`, fade + 4px translate.
- Modals: `--duration-slow`, fade + scale 0.96 → 1.
- No bounces, no springs, no playful easings.
- Respect `prefers-reduced-motion: reduce` by disabling all transitions.

---

## 3. Layout

Three-zone window:

```
┌──────────────────────────────────────────────────────────┐
│ Titlebar  36px  (drag region, custom Tauri decorations)  │
├──────────┬───────────────────────────────────────────────┤
│          │                                               │
│ Sidebar  │ Main content                                  │
│ 220px    │ (max-width 1100px, padding 24/32)             │
│          │                                               │
│          │                                               │
├──────────┴───────────────────────────────────────────────┤
│ Statusbar 28px (scope, dirty state, OpenCode version)    │
└──────────────────────────────────────────────────────────┘
```

**Sidebar.** Sections grouped: Agents, Providers, MCP Servers, Permissions, Commands, Settings. Items 32px tall, padding 8/12, optional 16×16 icon on the left.

**Statusbar.** Always-visible state: current scope (Global / Project), unsaved changes indicator (an `--accent` dot when dirty), detected OpenCode version. Crucial for orientation.

---

## 4. Components

### 4.1 Buttons

Three hierarchies plus one dangerous variant. Nothing else — no "ghost pill", no "subtle outline".

| Variant   | Background        | Text             | Border            | When                                    |
| --------- | ----------------- | ---------------- | ----------------- | --------------------------------------- |
| Primary   | `--accent`        | `--accent-text`  | none              | One per view: Save, Create, Connect     |
| Secondary | `--bg-2`          | `--text-primary` | `--border-default`| Default for most actions                |
| Ghost     | transparent       | `--text-secondary`| none             | Toolbars, near inputs, table headers    |
| Danger    | `--danger-muted`  | `--danger`       | `--danger`        | Delete, revoke, destructive             |

**Sizes**

| Size | Height | Padding (v/h) | Used in                          |
| ---- | ------ | ------------- | -------------------------------- |
| sm   | 28     | 4 / 12        | Table actions, inline            |
| md   | 32     | 6 / 14        | Default                          |
| lg   | 40     | 8 / 16        | Onboarding CTAs only             |

Loading state: spinner replaces icon (or appears left of text), button stays same width, disabled while loading.

### 4.2 Inputs

Height 32px, border `--border-default`, focus border `--accent` plus a 2px ring in `--accent-muted`. Radius `--radius-md`. Horizontal padding 10px.

**Variants**

- **TextInput** — basic.
- **CodeInput** — TextInput with `font-family: var(--font-mono)`. For model names, paths, commands.
- **PathInput** — CodeInput + "Browse" button glued to the right; opens Tauri's native dialog.
- **NumberInput** — with `+/-` steppers shown on hover only.
- **Select** — looks like an input; dropdown on `--bg-2`, items 28px tall.
- **Combobox** — Select with filter-as-you-type; required for long lists (models, providers).
- **Textarea** — for agent prompts. Min 6 lines, scrollable, max-height capped.
- **Toggle** — 28×16 switch, only for pure booleans.

**Slots per field**

- Label above (12px, weight 500).
- Helper below in `--text-tertiary` (11px).
- Error below in `--danger` (11px) — replaces helper when present.

### 4.3 Cards & panels

A **Card** is `--bg-1` over `--bg-0`, border `--border-subtle`, radius `--radius-lg`, internal padding 16/20.

A **Panel** is the same surface treatment but occupies a layout region (sidebar, main). No border when flush against the window edge; `--border-subtle` separating panels from each other.

### 4.4 Tables

For dense lists (agents, MCP servers, providers, models):

- Row height 36px (32 if you want even more density).
- Header `--bg-2`, sticky on scroll.
- Row hover: `--bg-2`.
- Selected row: `--bg-3` with a 2px left border in `--accent`.
- Row separators: `--border-subtle` — almost invisible. Heavy lines on every row are fatiguing.

### 4.5 Permission matrix

Loom's flagship component, with its own conventions:

- Rows = agents, columns = tools/permissions, cells 32×32.
- Each cell is a tri-state toggle:
  - **Allow** — bg `--success-muted`, check icon in `--success`.
  - **Ask** — bg `--warning-muted`, "?" icon in `--warning`.
  - **Deny** — bg `--danger-muted`, "×" icon in `--danger`.
- Click cycles through the three states.
- Shift-click applies the chosen state to the entire column.
- Header row sticky; first column (agent name) sticky.

### 4.6 Diff viewer

Shown before any save, no exceptions.

- Added lines: bg `--diff-add-bg`, 2px left border `--diff-add-border`, `+` prefix in `--success`.
- Removed lines: bg `--diff-remove-bg`, 2px left border `--diff-remove-border`, `-` prefix in `--danger`.
- Everything in `--font-mono`, 12px, line-height 1.5.

### 4.7 Status badges

Small pills for state (connected, error, draft, modified):

- Padding 2/8, radius `--radius-sm`, font-size 11px, weight 500.
- State variants pair semantic color + muted: e.g. `connected` → bg `--success-muted`, text `--success`, no border.

### 4.8 Toasts

- Position: bottom-right.
- Width 360px, padding 12/16, radius `--radius-md`, shadow `--shadow-md`.
- 3px left border in semantic color (success/warning/danger/info).
- Auto-dismiss after 4s; manual dismiss with `×` button.
- Stack vertically with 8px gap, max 3 visible.

---

## 5. State

State patterns that are easy to forget but matter most:

- **Empty state.** Every empty list has a placeholder with a short message and a CTA. Example: *"No agents configured yet — Create your first agent."*
- **Loading.** No full-screen spinners. Use skeleton loaders inside cards and rows. Inline spinners only inside buttons triggering an async action.
- **Error.** Banner above the affected content in `--danger-muted` / `--danger`, with a "Retry" or "Show details" action.
- **Dirty.** Any unsaved change paints an `--accent` dot next to the section title and in the statusbar. The Save button enables and shifts to Primary.
- **Disabled.** Opacity 0.5, cursor `not-allowed`, no hover. If disabled for a specific reason ("needs API key"), provide a tooltip explaining it.

---

## 6. Iconography

Use [Lucide](https://lucide.dev) via `lucide-svelte`. Sizes are fixed: 14, 16, 20. Stroke width always 2.

Conventions:

- 14px inside small badges and buttons.
- 16px inside default inputs and table cells.
- 20px in the sidebar and headers.

Do not mix icon libraries.

---

## 7. Accessibility

Minimum bar:

- AA contrast on every text/background pair. Tokens above are chosen to meet this; re-verify if you add new pairs.
- Focus is always visible. The default focus ring is a 2px `--accent` outline. Never remove `outline` without an equivalent replacement.
- Keyboard shortcuts for core actions:
  - `⌘S` — save
  - `⌘Z` / `⌘⇧Z` — undo / redo
  - `⌘\` — toggle sidebar
  - `⌘K` — command palette (phase 2)
  - `Esc` — close any modal/popover
- All interactive elements reachable via Tab, in logical order.
- Form fields associated with their labels via `for` / `id`.

---

## 8. Writing style

UI copy is part of the design system.

- Sentence case for everything. Never Title Case Buttons.
- Verbs in buttons, not nouns. "Save changes", not "Save changes button" or "Confirmation".
- No exclamation marks.
- Errors describe what happened and what to do, in that order. Bad: *"Something went wrong."* Good: *"Couldn't write to opencode.json — check file permissions and try again."*
- Use the user's terminology: "agent", "provider", "MCP server", "permission" — match OpenCode's own docs verbatim.

---

## 9. File layout

```
src/
├── app.css                       # tokens (this document → CSS)
├── lib/
│   └── components/
│       └── ui/                   # primitives built on tokens
│           ├── Button.svelte
│           ├── TextInput.svelte
│           ├── CodeInput.svelte
│           ├── Select.svelte
│           ├── Combobox.svelte
│           ├── Textarea.svelte
│           ├── Toggle.svelte
│           ├── Card.svelte
│           ├── Badge.svelte
│           ├── Toast.svelte
│           ├── DiffViewer.svelte
│           └── PermissionMatrix.svelte
└── routes/
    └── ...
```

**Rules**

- No third-party UI libraries (no shadcn, no Skeleton, no Flowbite). For an app this focused, owning ~10 primitives is cheaper than fighting library defaults.
- Components consume tokens only. No hardcoded colors, sizes, or radii.
- Every primitive has a single Svelte file with `<script>`, `<template>`, `<style>` — co-located, no per-component folders.

---

## 10. Changes to this document

This is a living document, but changes have weight. Before adding a new token, variant, or component:

1. Check whether an existing token/variant covers the case.
2. If not, propose the change in a PR that updates this document **and** the affected primitive in the same commit.
3. Avoid one-off values — if a value is used twice, it becomes a token.
