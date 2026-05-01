# Loom

Visual config editor for [OpenCode](https://opencode.ai). A small Tauri 2 +
SvelteKit desktop app that reads, edits and writes the OpenCode JSON config
without you ever opening `~/.config/opencode/opencode.json` by hand.

This branch (`claude/create-loom-editor-7VmM6`) is the bootstrap: project
scaffold, schema fetch, Rust backend skeleton, and a verification page that
proves frontend ↔ backend wiring works end-to-end.

## Stack

- **SvelteKit 2 + Svelte 5** (runes, SPA mode via `adapter-static`).
- **Tauri 2** as the desktop shell.
- **TypeScript** everywhere on the frontend.
- **`json-schema-to-typescript`** to compile the upstream OpenCode JSON Schema
  into typed `OpenCodeConfig` definitions.

## Layout

```
src/
  routes/                 SvelteKit pages
  lib/
    components/ui/        Reusable UI primitives (empty for now)
    schema/opencode.ts    Generated types — DO NOT EDIT BY HAND
    stores/               Svelte stores
    tauri/commands.ts     Typed wrappers around invoke()
src-tauri/
  src/
    lib.rs                #[tauri::command] entry points
    paths.rs              XDG-aware config path resolution
    config.rs             Atomic JSON read/write
scripts/
  fetch-schema.ts         Downloads opencode.ai/config.json and codegen
```

## Prerequisites

- Node.js ≥ 20 and `pnpm`
- Rust toolchain (`rustup` stable)
- Tauri 2 [system dependencies](https://tauri.app/start/prerequisites/)

## Install

```sh
pnpm install
```

The first install runs `pnpm schema:fetch` automatically (via `predev` /
`prebuild`). It downloads `https://opencode.ai/config.json`, caches it at
`src/lib/schema/opencode.schema.json`, and emits typed bindings to
`src/lib/schema/opencode.ts`. If the host is unreachable and there is no
cached schema, the script falls back to a permissive
`type OpenCodeConfig = Record<string, unknown>` stub so dev/build never block —
re-run `pnpm schema:fetch` once you're online to get real types.

## Develop

```sh
pnpm tauri:dev
```

This launches Vite on `http://localhost:1420` and opens the Tauri window.
The verification page calls four backend commands:

| Command         | Purpose                                            |
| --------------- | -------------------------------------------------- |
| `app_version`   | Sanity check that the IPC bridge works.            |
| `config_paths`  | Returns the global + project config paths and existence flags. |
| `read_config`   | Reads and parses `opencode.json` (or reports it missing). |
| `write_config`  | Atomically writes a new JSON value (used later).   |

Pure frontend dev (no Tauri shell):

```sh
pnpm dev
```

## Build

```sh
pnpm tauri:build      # full installer
pnpm build            # SvelteKit static bundle only
```

## Scripts

| Script             | Description                                  |
| ------------------ | -------------------------------------------- |
| `pnpm dev`         | SvelteKit dev server                         |
| `pnpm build`       | SvelteKit static build                       |
| `pnpm check`       | `svelte-kit sync` + `svelte-check`           |
| `pnpm schema:fetch`| Fetch + regenerate `OpenCodeConfig` types    |
| `pnpm tauri:dev`   | Tauri dev (frontend + Rust shell)            |
| `pnpm tauri:build` | Tauri release build                          |

## License

MIT
