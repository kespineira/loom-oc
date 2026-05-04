<script lang="ts">
  import { onMount, tick } from "svelte";
  import { RefreshCw, Sun, Moon, Save, X, Sidebar, Undo2, Redo2, Command as CommandIcon } from "lucide-svelte";
  import {
    appVersion,
    configPaths,
    readConfig,
    writeConfig,
    type ConfigPaths,
    type ConfigTarget,
  } from "$lib/tauri/commands";
  import { Badge, Button, Card, DiffViewer, SidebarNav, Toast } from "$lib/components/ui";
  import { theme } from "$lib/stores/theme.svelte";
  import { toasts } from "$lib/stores/toast.svelte";
  import { validateConfig, type ValidationIssue } from "$lib/schema/validate";
  import SettingsEditor from "$lib/components/editors/SettingsEditor.svelte";
  import AgentsEditor from "$lib/components/editors/AgentsEditor.svelte";
  import ProvidersEditor from "$lib/components/editors/ProvidersEditor.svelte";
  import McpEditor from "$lib/components/editors/McpEditor.svelte";
  import PermissionsEditor from "$lib/components/editors/PermissionsEditor.svelte";
  import CommandsEditor from "$lib/components/editors/CommandsEditor.svelte";
  import PluginsEditor from "$lib/components/editors/PluginsEditor.svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  let version = $state("…");
  let paths = $state<ConfigPaths | null>(null);
  let config = $state<OpenCodeConfig | null>(null);
  let savedConfig = $state<OpenCodeConfig | null>(null);
  let revision = $state<string | null>(null);
  let activeTarget = $state<ConfigTarget>("global");
  let error = $state<string | null>(null);
  let validationIssues = $state<ValidationIssue[]>([]);
  let loading = $state(true);
  let saving = $state(false);
  let activeTab = $state("settings");
  let showSaveConfirm = $state(false);
  let sidebarCollapsed = $state(false);
  let undoStack = $state<OpenCodeConfig[]>([]);
  let redoStack = $state<OpenCodeConfig[]>([]);
  let showCommandPalette = $state(false);
  let commandQuery = $state("");
  let commandInput = $state<HTMLInputElement | null>(null);

  type PaletteAction = {
    id: string;
    label: string;
    description: string;
    shortcut?: string;
    disabled: boolean;
    run: () => void | Promise<void>;
  };

  const isDirty = $derived(
    config !== null && savedConfig !== null && stringifyConfig(config) !== stringifyConfig(savedConfig),
  );

  const activePath = $derived(activeTarget === "global" ? paths?.global : paths?.project);
  const activeExists = $derived(activeTarget === "global" ? paths?.global_exists : paths?.project_exists);
  const paletteActions = $derived<PaletteAction[]>([
    {
      id: "save",
      label: "Save changes",
      description: "Validate and open the save review",
      shortcut: "⌘S",
      disabled: !config || !isDirty || saving,
      run: requestSave,
    },
    {
      id: "refresh",
      label: "Refresh config",
      description: "Reload the active config file from disk",
      shortcut: "⌘R",
      disabled: loading,
      run: () => refresh(activeTarget),
    },
    {
      id: "scope",
      label: `Switch to ${activeTarget === "global" ? "project" : "global"} config`,
      description: isDirty ? "Save or refresh before switching scope" : "Change the active config scope",
      disabled: isDirty,
      run: () => changeTarget(activeTarget === "global" ? "project" : "global"),
    },
    {
      id: "sidebar",
      label: "Toggle sidebar",
      description: sidebarCollapsed ? "Expand navigation" : "Collapse navigation",
      shortcut: "⌘\\",
      disabled: false,
      run: () => sidebarCollapsed = !sidebarCollapsed,
    },
    {
      id: "theme",
      label: "Toggle theme",
      description: "Switch between light and dark mode",
      disabled: false,
      run: () => theme.toggle(),
    },
    ...[
      ["settings", "Settings"],
      ["agents", "Agents"],
      ["providers", "Providers"],
      ["mcp", "MCP Servers"],
      ["permissions", "Permissions"],
      ["commands", "Commands"],
      ["plugins", "Plugins"],
    ].map(([id, label], index) => ({
      id: `section-${id}`,
      label: `Open ${label}`,
      description: "Switch editor section",
      shortcut: `⌘${index + 1}`,
      disabled: false,
      run: () => setActiveTab(id),
    })),
  ]);
  const shownPaletteActions = $derived(
    paletteActions.filter((action) => {
      const haystack = `${action.label} ${action.description}`.toLowerCase();
      return haystack.includes(commandQuery.trim().toLowerCase());
    }),
  );

  function setActiveTab(tab: string) {
    activeTab = tab;
  }

  async function openCommandPalette() {
    commandQuery = "";
    showCommandPalette = true;
    await tick();
    commandInput?.focus();
  }

  async function runPaletteAction(action: (typeof paletteActions)[number]) {
    if (action.disabled) return;
    showCommandPalette = false;
    commandQuery = "";
    await action.run();
  }

  function stringifyConfig(value: OpenCodeConfig | null) {
    return JSON.stringify(value ?? {}, null, 2);
  }

  function cloneConfig(value: OpenCodeConfig): OpenCodeConfig {
    return JSON.parse(JSON.stringify(value)) as OpenCodeConfig;
  }

  async function refresh(target: ConfigTarget = activeTarget) {
    loading = true;
    error = null;
    try {
      version = await appVersion();
      paths = await configPaths();
      const read = await readConfig(target);
      const nextConfig = read.value ?? { $schema: "https://opencode.ai/config.json" };
      config = cloneConfig(nextConfig);
      savedConfig = cloneConfig(nextConfig);
      revision = read.revision;
      validationIssues = [];
      undoStack = [];
      redoStack = [];
      showSaveConfirm = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      toasts.danger("Couldn't read config", error);
    } finally {
      loading = false;
    }
  }

  async function changeTarget(target: ConfigTarget) {
    if (target === activeTarget) return;
    if (isDirty) {
      toasts.warning("Unsaved changes", "Save or refresh before switching config scope.");
      return;
    }
    activeTarget = target;
    await refresh(target);
  }

  function requestSave() {
    if (!config || !paths || !isDirty) return;
    const validation = validateConfig(config);
    validationIssues = validation.issues;
    if (!validation.valid) {
      showSaveConfirm = false;
      toasts.danger("Config validation failed", "Fix validation errors before saving.");
      return;
    }
    showSaveConfirm = true;
  }

  async function confirmSave() {
    if (!config || !paths) return;
    const validation = validateConfig(config);
    validationIssues = validation.issues;
    if (!validation.valid) {
      showSaveConfirm = false;
      toasts.danger("Config validation failed", "Fix validation errors before saving.");
      return;
    }
    saving = true;
    try {
      const result = await writeConfig(activeTarget, config, revision);
      savedConfig = cloneConfig(config);
      revision = result.revision;
      paths = {
        ...paths,
        global_exists: activeTarget === "global" ? true : paths.global_exists,
        project_exists: activeTarget === "project" ? true : paths.project_exists,
      };
      showSaveConfirm = false;
      validationIssues = [];
      toasts.success("Config saved", result.path);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      toasts.danger("Couldn't save config", error);
    } finally {
      saving = false;
    }
  }

  function updateConfig(newConfig: OpenCodeConfig) {
    if (config && stringifyConfig(config) !== stringifyConfig(newConfig)) {
      undoStack = [...undoStack.slice(-99), cloneConfig(config)];
      redoStack = [];
    }
    config = newConfig;
    validationIssues = [];
  }

  function undoConfig() {
    if (!config || !undoStack.length) return;
    const previous = undoStack[undoStack.length - 1];
    undoStack = undoStack.slice(0, -1);
    redoStack = [...redoStack, cloneConfig(config)];
    config = cloneConfig(previous);
    validationIssues = [];
  }

  function redoConfig() {
    if (!config || !redoStack.length) return;
    const next = redoStack[redoStack.length - 1];
    redoStack = redoStack.slice(0, -1);
    undoStack = [...undoStack, cloneConfig(config)];
    config = cloneConfig(next);
    validationIssues = [];
  }

  function isEditingText(target: EventTarget | null) {
    return target instanceof HTMLInputElement
      || target instanceof HTMLTextAreaElement
      || target instanceof HTMLSelectElement
      || (target instanceof HTMLElement && target.isContentEditable);
  }

  function handleKeydown(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    const isModifier = event.metaKey || event.ctrlKey;
    if ((event.metaKey || event.ctrlKey) && key === "s") {
      event.preventDefault();
      requestSave();
    }
    if (isModifier && key === "k") {
      event.preventDefault();
      void openCommandPalette();
    }
    if (isModifier && /^[1-7]$/.test(key)) {
      event.preventDefault();
      const tabs = ["settings", "agents", "providers", "mcp", "permissions", "commands", "plugins"];
      setActiveTab(tabs[Number(key) - 1]);
    }
    if (isModifier && key === "z" && !isEditingText(event.target)) {
      event.preventDefault();
      if (event.shiftKey) redoConfig();
      else undoConfig();
    }
    if (isModifier && key === "y" && !isEditingText(event.target)) {
      event.preventDefault();
      redoConfig();
    }
    if (isModifier && key === "\\") {
      event.preventDefault();
      sidebarCollapsed = !sidebarCollapsed;
    }
    if (event.key === "Escape") {
      showCommandPalette = false;
      showSaveConfirm = false;
    }
  }

  onMount(() => {
    refresh(activeTarget);
    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });
</script>

<svelte:head>
  <title>Loom — Visual Config Editor</title>
</svelte:head>

<main class="shell">
  <SidebarNav active={activeTab} dirty={isDirty} collapsed={sidebarCollapsed} onSelect={setActiveTab} />

  <section class="workspace">
  <header class="page__header">
    <div>
      <h1 class="page__title">Loom <span class="page__version">v{version}</span></h1>
      <p class="page__subtitle">
        Visual config editor for OpenCode
      </p>
    </div>
    <div class="page__toolbar">
      <Button
        variant="ghost"
        size="sm"
        aria-label="Toggle sidebar"
        onclick={() => sidebarCollapsed = !sidebarCollapsed}
      >
        {#snippet iconLeft()}<Sidebar size={16} />{/snippet}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        aria-label="Open command palette"
        onclick={openCommandPalette}
      >
        {#snippet iconLeft()}<CommandIcon size={16} />{/snippet}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        aria-label="Undo config change"
        onclick={undoConfig}
        disabled={!undoStack.length}
      >
        {#snippet iconLeft()}<Undo2 size={16} />{/snippet}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        aria-label="Redo config change"
        onclick={redoConfig}
        disabled={!redoStack.length}
      >
        {#snippet iconLeft()}<Redo2 size={16} />{/snippet}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        aria-label="Toggle theme"
        onclick={() => theme.toggle()}
      >
        {#snippet iconLeft()}
          {#if theme.value === "dark"}
            <Sun size={16} />
          {:else}
            <Moon size={16} />
          {/if}
        {/snippet}
      </Button>
      <Button
        variant="secondary"
        size="sm"
        onclick={() => refresh(activeTarget)}
        {loading}
      >
        {#snippet iconLeft()}<RefreshCw size={14} />{/snippet}
        Refresh
      </Button>
      <Button
        variant="primary"
        size="sm"
        onclick={requestSave}
        loading={saving}
        disabled={!config || !isDirty || saving}
      >
        {#snippet iconLeft()}<Save size={14} />{/snippet}
        Save{isDirty ? " *" : ""}
      </Button>
    </div>
  </header>

  {#if error}
    <Card title="Error" tone="danger">
      <pre class="errbox">{error}</pre>
    </Card>
  {/if}

  {#if validationIssues.length}
    <Card title="Validation errors" tone="danger">
      <ul class="validation-list">
        {#each validationIssues as issue}
          <li><code>{issue.path}</code> {issue.message}</li>
        {/each}
      </ul>
    </Card>
  {/if}

  {#if paths}
    <Card title="Config path" description="Editing {activeTarget} config.">
      <div class="scope-toggle" aria-label="Config scope">
        <button class:active={activeTarget === "global"} type="button" onclick={() => changeTarget("global")}>Global</button>
        <button class:active={activeTarget === "project"} type="button" onclick={() => changeTarget("project")}>Project</button>
      </div>
      <code>{activePath ?? "(unknown path)"}</code>
      {#if activeExists}
        <Badge tone="success">exists</Badge>
      {:else}
        <Badge tone="neutral">will be created on save</Badge>
      {/if}
      {#if isDirty}
        <Badge tone="warning">unsaved changes</Badge>
      {/if}
    </Card>
  {/if}

  {#if config}
    <Card class="editor-card">
      <div class="editor">
        {#if activeTab === "settings"}
          <SettingsEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "agents"}
          <AgentsEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "providers"}
          <ProvidersEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "mcp"}
          <McpEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "permissions"}
          <PermissionsEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "commands"}
          <CommandsEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "plugins"}
          <PluginsEditor config={config} onChange={updateConfig} />
        {/if}
      </div>
    </Card>
  {:else if !loading}
    <Card title="No config">
      <p class="muted">No config found at <code>{activePath}</code>. Click Refresh to check again.</p>
    </Card>
  {/if}

  {#if showSaveConfirm && savedConfig && config}
    <div class="modal-backdrop" role="presentation">
      <div class="modal" role="dialog" aria-modal="true" aria-labelledby="save-confirm-title">
        <header class="modal__header">
          <div>
            <h2 id="save-confirm-title">Confirm save</h2>
            <p>Review changes before writing <code>{activePath}</code>.</p>
          </div>
          <Button variant="ghost" size="sm" aria-label="Close" onclick={() => showSaveConfirm = false} disabled={saving}>
            <X size={14} />
          </Button>
        </header>

        <div class="modal__diff">
          <DiffViewer before={savedConfig} after={config} />
        </div>

        <footer class="modal__footer">
          <Button variant="secondary" onclick={() => showSaveConfirm = false} disabled={saving}>Cancel</Button>
          <Button variant="primary" onclick={confirmSave} loading={saving}>Save changes</Button>
        </footer>
      </div>
    </div>
  {/if}

  {#if showCommandPalette}
    <div class="modal-backdrop" role="presentation">
      <div class="command-palette" role="dialog" aria-modal="true" aria-labelledby="command-palette-title">
        <header class="command-palette__header">
          <CommandIcon size={16} />
          <label id="command-palette-title" for="command-search">Command palette</label>
          <input
            id="command-search"
            bind:this={commandInput}
            bind:value={commandQuery}
            placeholder="Search actions and sections"
            onkeydown={(event) => {
              if (event.key === "Enter" && shownPaletteActions[0]) void runPaletteAction(shownPaletteActions[0]);
            }}
          />
        </header>
        <div class="command-palette__list">
          {#if shownPaletteActions.length}
            {#each shownPaletteActions as action (action.id)}
              <button type="button" disabled={action.disabled} onclick={() => runPaletteAction(action)}>
                <span>
                  <strong>{action.label}</strong>
                  <small>{action.description}</small>
                </span>
                {#if action.shortcut}<kbd>{action.shortcut}</kbd>{/if}
              </button>
            {/each}
          {:else}
            <p>No matching command.</p>
          {/if}
        </div>
      </div>
    </div>
  {/if}
  <footer class="statusbar">
    <span>Scope: {activeTarget}</span>
    <span>{activePath ?? "No path"}</span>
    <span>OpenCode config {activeExists ? "found" : "missing"}</span>
    <span class:dirty={isDirty}>{isDirty ? "Unsaved changes" : "Saved"}</span>
    <span>Undo {undoStack.length} / Redo {redoStack.length}</span>
    <span>Loom v{version}</span>
  </footer>
  </section>
</main>

<Toast />

<style>
  .shell {
    min-height: 100vh;
    display: flex;
    background: var(--bg-0);
  }

  .workspace {
    min-width: 0;
    height: 100vh;
    flex: 1;
    padding: var(--space-6) var(--space-5) var(--space-8);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    overflow: auto;
  }

  .page__header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-4);
  }

  .page__title {
    margin: 0;
    font-size: var(--text-2xl);
    font-weight: 600;
    line-height: var(--leading-tight);
  }

  .page__version {
    margin-left: var(--space-2);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 400;
    color: var(--text-tertiary);
  }

  .page__subtitle {
    margin: var(--space-1) 0 0;
    font-size: var(--text-base);
    color: var(--text-secondary);
  }

  .page__toolbar {
    display: flex;
    gap: var(--space-2);
    flex-shrink: 0;
  }

  code {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    background: var(--bg-2);
    color: var(--text-primary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .scope-toggle {
    display: inline-flex;
    gap: 2px;
    padding: 2px;
    background: var(--bg-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
  }

  .scope-toggle button {
    height: 24px;
    padding: 0 var(--space-2);
    background: transparent;
    border: 0;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    cursor: pointer;
    font-family: var(--font-sans);
    font-size: var(--text-sm);
  }

  .scope-toggle button.active {
    background: var(--accent);
    color: var(--accent-text);
  }

  .errbox {
    margin: 0;
    padding: var(--space-3) var(--space-4);
    background: var(--bg-0);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    line-height: 1.5;
    overflow: auto;
    max-height: 24rem;
    color: var(--danger);
  }

  .validation-list {
    margin: 0;
    padding-left: var(--space-4);
    color: var(--danger);
    font-size: var(--text-sm);
  }

  .validation-list li + li {
    margin-top: var(--space-1);
  }

  .muted {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .editor {
    padding: var(--space-4);
  }

  :global(.editor-card) {
    padding: 0;
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 50;
    display: grid;
    place-items: center;
    padding: var(--space-5);
    background: color-mix(in oklab, black 62%, transparent);
  }

  .modal {
    width: min(900px, 100%);
    max-height: min(760px, 90vh);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-5);
    background: var(--bg-0);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
  }

  .modal__header,
  .modal__footer {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-3);
  }

  .modal__header h2 {
    margin: 0;
    font-size: var(--text-xl);
    line-height: var(--leading-tight);
  }

  .modal__header p {
    margin: var(--space-1) 0 0;
    color: var(--text-secondary);
    font-size: var(--text-sm);
  }

  .modal__diff {
    min-height: 0;
    overflow: auto;
  }

  .modal__footer {
    align-items: center;
    justify-content: flex-end;
  }

  .command-palette {
    width: min(640px, 100%);
    max-height: min(620px, 86vh);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg-0);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
  }

  .command-palette__header {
    display: grid;
    grid-template-columns: auto auto 1fr;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-secondary);
  }

  .command-palette__header label {
    color: var(--text-primary);
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .command-palette__header input {
    min-width: 0;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font: inherit;
    outline: none;
  }

  .command-palette__list {
    padding: var(--space-2);
    overflow: auto;
  }

  .command-palette__list button {
    width: 100%;
    min-height: 54px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-3);
    background: transparent;
    border: 0;
    border-radius: var(--radius-md);
    color: var(--text-primary);
    cursor: pointer;
    font-family: var(--font-sans);
    text-align: left;
  }

  .command-palette__list button:hover:not(:disabled),
  .command-palette__list button:focus-visible {
    background: var(--bg-2);
    outline: none;
  }

  .command-palette__list button:disabled {
    cursor: not-allowed;
    opacity: 0.45;
  }

  .command-palette__list span,
  .command-palette__list strong,
  .command-palette__list small {
    display: block;
  }

  .command-palette__list strong {
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .command-palette__list small {
    margin-top: 2px;
    color: var(--text-tertiary);
    font-size: var(--text-xs);
  }

  .command-palette__list p {
    margin: 0;
    padding: var(--space-4);
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  kbd {
    flex-shrink: 0;
    padding: 2px 6px;
    background: var(--bg-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }

  .statusbar {
    position: sticky;
    bottom: calc(-1 * var(--space-8));
    height: 28px;
    margin: auto calc(-1 * var(--space-5)) calc(-1 * var(--space-8));
    padding: 0 var(--space-4);
    display: flex;
    align-items: center;
    gap: var(--space-4);
    background: var(--bg-1);
    border-top: 1px solid var(--border-subtle);
    color: var(--text-tertiary);
    font-size: var(--text-xs);
  }

  .statusbar span:nth-child(2) {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .statusbar .dirty {
    color: var(--accent);
  }
</style>
