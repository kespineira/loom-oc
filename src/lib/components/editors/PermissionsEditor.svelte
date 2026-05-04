<script lang="ts">
  import { PermissionMatrix, Select, Textarea } from "$lib/components/ui";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  type PermissionState = "allow" | "ask" | "deny";
  type PermissionObject = Record<string, PermissionState | Record<string, PermissionState>>;
  type PermissionValue = PermissionState | PermissionObject;

  let { config, onChange }: Props = $props();
  let jsonError = $state<string | null>(null);

  const knownTools = [
    "read",
    "edit",
    "glob",
    "grep",
    "list",
    "bash",
    "task",
    "external_directory",
    "todowrite",
    "question",
    "webfetch",
    "websearch",
    "lsp",
    "skill",
    "doom_loop",
  ];

  const rows = knownTools.map((tool) => ({ id: tool, label: tool }));
  const columns = [{ id: "default", label: "default" }];

  const permission = $derived(config.permission as PermissionValue | undefined);
  const globalMode = $derived(typeof permission === "string" ? permission : "custom");
  const matrixValue = $derived(toMatrix(permission));
  const rawJson = $derived(JSON.stringify(permission ?? {}, null, 2));

  function toMatrix(value: PermissionValue | undefined) {
    const matrix: Record<string, Record<string, PermissionState>> = {};
    for (const tool of knownTools) {
      const rule = typeof value === "object" && value ? value[tool] : undefined;
      matrix[tool] = {
        default: typeof rule === "string" ? rule : "ask",
      };
    }
    return matrix;
  }

  function commit(value: PermissionValue | undefined) {
    onChange({ ...config, permission: value as OpenCodeConfig["permission"] });
  }

  function setGlobalMode(value: string) {
    jsonError = null;
    if (!value) commit(undefined);
    else if (value === "custom") commit(typeof permission === "object" ? permission : {});
    else commit(value as PermissionState);
  }

  function setMatrix(next: Record<string, Record<string, PermissionState>>) {
    const current = typeof permission === "object" && permission ? permission : {};
    const result: PermissionObject = { ...current };
    for (const tool of knownTools) {
      result[tool] = next[tool]?.default ?? "ask";
    }
    commit(result);
  }

  function updateRaw(value: string) {
    try {
      const parsed = JSON.parse(value) as PermissionValue;
      jsonError = null;
      commit(parsed);
    } catch (error) {
      jsonError = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<div class="permissions">
  <div class="field">
    <label for="global-permission">Global permission mode</label>
    <Select
      id="global-permission"
      value={globalMode}
      placeholder="Unset"
      options={[
        { value: "ask", label: "ask" },
        { value: "allow", label: "allow" },
        { value: "deny", label: "deny" },
        { value: "custom", label: "custom per tool" },
      ]}
      onchange={(event) => setGlobalMode((event.currentTarget as HTMLSelectElement).value)}
    />
  </div>

  {#if globalMode === "custom"}
    <div class="section">
      <h3>Tool defaults</h3>
      <p>Click to cycle allow → ask → deny. Shift-click applies a value to the column.</p>
      <PermissionMatrix rows={rows} {columns} value={matrixValue} onChange={setMatrix} />
    </div>

    <div class="field">
      <label for="permission-json">Advanced permission JSON</label>
      <Textarea
        id="permission-json"
        value={rawJson}
        mono
        rows={10}
        oninput={(event) => updateRaw((event.currentTarget as HTMLTextAreaElement).value)}
      />
      {#if jsonError}<p class="error">{jsonError}</p>{/if}
    </div>
  {/if}
</div>

<style>
  .permissions,
  .section,
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .field {
    gap: var(--space-1);
  }

  .field label,
  .section h3 {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .section p {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .error {
    margin: 0;
    color: var(--danger);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }
</style>
