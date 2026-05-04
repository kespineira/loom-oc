<script lang="ts">
  import { onMount } from "svelte";
  import { RefreshCw, Sun, Moon, Save, X } from "lucide-svelte";
  import {
    appVersion,
    configPaths,
    readConfig,
    writeConfig,
    type ConfigPaths,
  } from "$lib/tauri/commands";
  import { TabNav } from "$lib/components/ui";
  import { Badge, Button, Card, DiffViewer, Toast } from "$lib/components/ui";
  import { theme } from "$lib/stores/theme.svelte";
  import { toasts } from "$lib/stores/toast.svelte";
  import SettingsEditor from "$lib/components/editors/SettingsEditor.svelte";
  import ProvidersEditor from "$lib/components/editors/ProvidersEditor.svelte";
  import McpEditor from "$lib/components/editors/McpEditor.svelte";
  import PluginsEditor from "$lib/components/editors/PluginsEditor.svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  let version = $state("…");
  let paths = $state<ConfigPaths | null>(null);
  let config = $state<OpenCodeConfig | null>(null);
  let savedConfig = $state<OpenCodeConfig | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let activeTab = $state("settings");
  let showSaveConfirm = $state(false);

  const isDirty = $derived(
    config !== null && savedConfig !== null && stringifyConfig(config) !== stringifyConfig(savedConfig),
  );

  function setActiveTab(tab: string) {
    activeTab = tab;
  }

  function stringifyConfig(value: OpenCodeConfig | null) {
    return JSON.stringify(value ?? {}, null, 2);
  }

  function cloneConfig(value: OpenCodeConfig): OpenCodeConfig {
    return JSON.parse(JSON.stringify(value)) as OpenCodeConfig;
  }

  async function refresh() {
    loading = true;
    error = null;
    try {
      version = await appVersion();
      paths = await configPaths();
      const read = await readConfig(paths.global);
      const nextConfig = read.value ?? { $schema: "https://opencode.ai/config.json" };
      config = cloneConfig(nextConfig);
      savedConfig = cloneConfig(nextConfig);
      showSaveConfirm = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      toasts.danger("Couldn't read config", error);
    } finally {
      loading = false;
    }
  }

  function requestSave() {
    if (!config || !paths || !isDirty) return;
    showSaveConfirm = true;
  }

  async function confirmSave() {
    if (!config || !paths) return;
    saving = true;
    try {
      await writeConfig(paths.global, config);
      savedConfig = cloneConfig(config);
      paths = { ...paths, global_exists: true };
      showSaveConfirm = false;
      toasts.success("Config saved", paths.global);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      toasts.danger("Couldn't save config", error);
    } finally {
      saving = false;
    }
  }

  function updateConfig(newConfig: OpenCodeConfig) {
    config = newConfig;
  }

  function handleKeydown(event: KeyboardEvent) {
    const key = event.key.toLowerCase();
    if ((event.metaKey || event.ctrlKey) && key === "s") {
      event.preventDefault();
      requestSave();
    }
    if (event.key === "Escape") {
      showSaveConfirm = false;
    }
  }

  onMount(() => {
    refresh();
    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });
</script>

<svelte:head>
  <title>Loom — Visual Config Editor</title>
</svelte:head>

<main class="page">
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
        onclick={refresh}
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

  {#if paths}
    <Card title="Config path" description="Editing global config.">
      <code>{paths.global}</code>
      {#if paths.global_exists}
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
      <TabNav active={activeTab} onTabChange={setActiveTab} />

      <div class="editor">
        {#if activeTab === "settings"}
          <SettingsEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "providers"}
          <ProvidersEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "mcp"}
          <McpEditor config={config} onChange={updateConfig} />
        {:else if activeTab === "plugins"}
          <PluginsEditor config={config} onChange={updateConfig} />
        {/if}
      </div>
    </Card>
  {:else if !loading}
    <Card title="No config">
      <p class="muted">No config found at <code>{paths?.global}</code>. Click Refresh to check again.</p>
    </Card>
  {/if}

  {#if showSaveConfirm && savedConfig && config}
    <div class="modal-backdrop" role="presentation">
      <div class="modal" role="dialog" aria-modal="true" aria-labelledby="save-confirm-title">
        <header class="modal__header">
          <div>
            <h2 id="save-confirm-title">Confirm save</h2>
            <p>Review changes before writing <code>{paths?.global}</code>.</p>
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
</main>

<Toast />

<style>
  .page {
    max-width: 880px;
    margin: 0 auto;
    padding: var(--space-6) var(--space-5) var(--space-8);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
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

  .muted {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .editor {
    padding-top: var(--space-4);
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
</style>
