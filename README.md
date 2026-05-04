# Loom

Visual config editor for [OpenCode](https://opencode.ai). Loom is a small Tauri 2 and SvelteKit desktop app for editing OpenCode JSON configuration without opening `~/.config/opencode/opencode.json` by hand.

## Features

- Global and project config scopes with backend-resolved paths.
- Visual editors for settings, agents, providers, MCP servers, permissions, commands, and plugins.
- Dirty-state tracking, undo/redo, and a diff review before every save.
- Schema validation using the cached upstream OpenCode JSON Schema before writing to disk.
- Conflict detection through file revisions so external edits are not overwritten silently.
- Automatic `.bak` backup before overwriting an existing config file.
- Light and dark themes with a collapsible sidebar and command palette.

## Shortcuts

| Shortcut | Action |
| --- | --- |
| `Cmd+K` / `Ctrl+K` | Open the command palette |
| `Cmd+S` / `Ctrl+S` | Validate and review save changes |
| `Cmd+Z` / `Ctrl+Z` | Undo config change when focus is outside a text field |
| `Cmd+Shift+Z` / `Ctrl+Shift+Z` | Redo config change when focus is outside a text field |
| `Cmd+\` / `Ctrl+\` | Toggle the sidebar |
| `Cmd+1` through `Cmd+7` | Jump between editor sections |
| `Esc` | Close dialogs |

## Stack

- SvelteKit 2 + Svelte 5 in SPA mode via `adapter-static`.
- Tauri 2 as the desktop shell.
- TypeScript on the frontend.
- Rust backend commands for path resolution, JSON read/write, backups, and conflict checks.
- `json-schema-to-typescript` to compile the upstream OpenCode JSON Schema into `OpenCodeConfig` definitions.
- Ajv 2020 for runtime schema validation.

## Layout

```text
src/
  routes/                 SvelteKit app shell
  lib/
    components/editors/   Visual OpenCode config editors
    components/ui/        Reusable UI primitives
    schema/               Cached schema, generated types, validation helpers
    stores/               Svelte stores
    tauri/commands.ts     Typed wrappers around invoke()
src-tauri/
  src/
    lib.rs                #[tauri::command] entry points
    paths.rs              XDG-aware config path resolution
    config.rs             JSON read/write, revisions, backups, conflict checks
scripts/
  fetch-schema.ts         Downloads opencode.ai/config.json and codegen
```

## Prerequisites

- Node.js 20 or newer and `pnpm`.
- Rust stable toolchain through `rustup`.
- Tauri 2 [system dependencies](https://tauri.app/start/prerequisites/).

## Install

```sh
pnpm install
```

Schema generation runs before dev/build through `pnpm schema:fetch`. It downloads `https://opencode.ai/config.json`, caches it at `src/lib/schema/opencode.schema.json`, and emits typed bindings to `src/lib/schema/opencode.ts`. If the host is unreachable and there is no cached schema, the script falls back to a permissive `type OpenCodeConfig = Record<string, unknown>` stub so local development can continue.

## Develop

```sh
pnpm tauri:dev
```

Pure frontend dev without the Tauri shell:

```sh
pnpm dev
```

## Build

```sh
pnpm tauri:build      # full installer
pnpm build            # SvelteKit static bundle only
```

## Verification

```sh
pnpm check
pnpm test
pnpm build
cd src-tauri && cargo test --lib
```

If your Rust toolchain has formatting and lint components installed, also run:

```sh
cd src-tauri && cargo fmt --all -- --check
cd src-tauri && cargo clippy --all-targets -- -D warnings
```

## Scripts

| Script | Description |
| --- | --- |
| `pnpm dev` | SvelteKit dev server |
| `pnpm build` | SvelteKit static build |
| `pnpm check` | `svelte-kit sync` plus `svelte-check` |
| `pnpm test` | Vitest unit tests |
| `pnpm schema:fetch` | Fetch and regenerate `OpenCodeConfig` types |
| `pnpm tauri:dev` | Tauri dev app |
| `pnpm tauri:build` | Tauri release build |

## License

MIT
