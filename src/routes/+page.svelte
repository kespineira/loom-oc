<script lang="ts">
  import { onMount } from "svelte";
  import { RefreshCw, Sun, Moon } from "lucide-svelte";
  import {
    appVersion,
    configPaths,
    readConfig,
    type ConfigPaths,
    type ReadResult,
  } from "$lib/tauri/commands";
  import {
    Badge,
    Button,
    Card,
    DiffViewer,
    PermissionMatrix,
    Toast,
  } from "$lib/components/ui";
  import { theme } from "$lib/stores/theme.svelte";
  import { toasts } from "$lib/stores/toast.svelte";

  let version = $state("…");
  let paths = $state<ConfigPaths | null>(null);
  let read = $state<ReadResult | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(true);

  // Permission matrix demo
  const agents = [
    { id: "build", label: "build" },
    { id: "review", label: "review" },
    { id: "plan", label: "plan" },
  ];
  const tools = [
    { id: "edit", label: "edit" },
    { id: "bash", label: "bash" },
    { id: "webfetch", label: "webfetch" },
    { id: "write", label: "write" },
  ];
  let permissions = $state<Record<string, Record<string, "allow" | "ask" | "deny">>>({
    build: { edit: "allow", bash: "ask", webfetch: "deny", write: "allow" },
    review: { edit: "deny", bash: "deny", webfetch: "ask", write: "deny" },
    plan: { edit: "deny", bash: "deny", webfetch: "ask", write: "deny" },
  });

  async function refresh() {
    loading = true;
    error = null;
    try {
      version = await appVersion();
      paths = await configPaths();
      read = await readConfig(paths.global);
      toasts.success("Config refreshed", paths.global);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      toasts.danger("Couldn't read config", error);
    } finally {
      loading = false;
    }
  }

  onMount(refresh);
</script>

<svelte:head>
  <title>Loom — verification</title>
</svelte:head>

<main class="page">
  <header class="page__header">
    <div>
      <h1 class="page__title">Loom <span class="page__version">v{version}</span></h1>
      <p class="page__subtitle">
        Visual config editor for OpenCode — bootstrap verification page.
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
      <Button variant="secondary" size="sm" onclick={refresh} {loading}>
        {#snippet iconLeft()}<RefreshCw size={14} />{/snippet}
        Refresh
      </Button>
    </div>
  </header>

  {#if error}
    <Card title="Error" tone="danger">
      <pre class="errbox">{error}</pre>
    </Card>
  {/if}

  {#if paths}
    <Card title="Config paths" description="Resolved by the Rust backend (XDG-aware).">
      <dl class="paths">
        <dt>Global</dt>
        <dd>
          <code>{paths.global}</code>
          <Badge tone={paths.global_exists ? "success" : "neutral"}>
            {paths.global_exists ? "exists" : "missing"}
          </Badge>
        </dd>
        <dt>Project</dt>
        <dd>
          <code>{paths.project ?? "(unknown cwd)"}</code>
          {#if paths.project}
            <Badge tone={paths.project_exists ? "success" : "neutral"}>
              {paths.project_exists ? "exists" : "missing"}
            </Badge>
          {/if}
        </dd>
      </dl>
    </Card>
  {/if}

  {#if read}
    <Card title="Global config (raw)">
      {#if read.exists && read.raw}
        <pre class="raw">{read.raw}</pre>
      {:else}
        <p class="muted">No config file at <code>{read.path}</code> yet.</p>
      {/if}
    </Card>
  {/if}

  <Card
    title="Diff viewer"
    description="Sample comparison; the editor will use this before every save."
  >
    <DiffViewer
      before={{ model: "anthropic/claude-sonnet-4-5", theme: "dark" }}
      after={{ model: "anthropic/claude-opus-4-7", theme: "dark", autoshare: false }}
    />
  </Card>

  <Card
    title="Permission matrix"
    description="Click to cycle allow → ask → deny. Shift-click to apply to a column."
  >
    <PermissionMatrix rows={agents} columns={tools} bind:value={permissions} />
  </Card>
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

  .paths {
    display: grid;
    grid-template-columns: max-content 1fr;
    column-gap: var(--space-4);
    row-gap: var(--space-2);
    margin: 0;
  }

  .paths dt {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-secondary);
  }

  .paths dd {
    margin: 0;
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  code {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    background: var(--bg-2);
    color: var(--text-primary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .raw,
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
  }

  .errbox {
    color: var(--danger);
  }

  .muted {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }
</style>
