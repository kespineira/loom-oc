<script lang="ts">
  import { Badge, Button, TextInput, Toggle } from "$lib/components/ui";
  import { ChevronDown, ChevronRight, Plus, Trash2 } from "lucide-svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  type McpRemote = {
    type: "remote";
    url: string;
    enabled?: boolean;
    headers?: Record<string, string>;
    timeout?: number;
  };

  type McpLocal = {
    type: "local";
    command: string[];
    enabled?: boolean;
    environment?: Record<string, string>;
    timeout?: number;
  };

  type McpEntry = McpRemote | McpLocal | { enabled: boolean };
  type McpServer = { name: string; entry: McpEntry };

  let { config, onChange }: Props = $props();

  const servers: McpServer[] = $derived(
    config.mcp
      ? Object.entries(config.mcp).map(([name, entry]) => ({ name, entry: entry as McpEntry }))
      : [],
  );

  let expandedServers = $state<Set<string>>(new Set());
  let newServerName = $state("");
  let newServerType = $state<"remote" | "local">("remote");
  let newServerUrl = $state("");
  let newServerCommand = $state("");
  let newServerEnabled = $state(true);

  function commit(nextServers: McpServer[]) {
    const mcp: Record<string, unknown> = {};
    for (const server of nextServers) mcp[server.name] = server.entry;
    onChange({ ...config, mcp: Object.keys(mcp).length ? mcp as OpenCodeConfig["mcp"] : undefined });
  }

  function addServer() {
    const name = newServerName.trim().toLowerCase();
    if (!name || servers.some((server) => server.name === name)) {
      newServerName = "";
      return;
    }

    const entry: McpEntry = newServerType === "remote"
      ? { type: "remote", url: newServerUrl.trim(), enabled: newServerEnabled }
      : {
          type: "local",
          command: newServerCommand.split(" ").map((part) => part.trim()).filter(Boolean),
          enabled: newServerEnabled,
        };

    commit([...servers, { name, entry }]);
    expandedServers = new Set([...expandedServers, name]);
    newServerName = "";
    newServerUrl = "";
    newServerCommand = "";
    newServerEnabled = true;
  }

  function removeServer(name: string) {
    commit(servers.filter((server) => server.name !== name));
    expandedServers = new Set([...expandedServers].filter((key) => key !== name));
  }

  function updateServer(name: string, update: (entry: McpEntry) => McpEntry) {
    commit(
      servers.map((server) => server.name === name ? { ...server, entry: update(server.entry) } : server),
    );
  }

  function toggleServer(name: string) {
    const next = new Set(expandedServers);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    expandedServers = next;
  }

  function serverType(entry: McpEntry) {
    return "type" in entry ? entry.type : "toggle";
  }
</script>

<div class="mcp">
  <div class="mcp__list">
    {#each servers as server (server.name)}
      <div class="server">
        <button
          type="button"
          class="server__header"
          aria-expanded={expandedServers.has(server.name)}
          onclick={() => toggleServer(server.name)}
        >
          <span class="server__toggle" aria-hidden="true">
            {#if expandedServers.has(server.name)}
              <ChevronDown size={14} />
            {:else}
              <ChevronRight size={14} />
            {/if}
          </span>
          <span class="server__name">{server.name}</span>
          <Badge tone={serverType(server.entry) === "remote" ? "neutral" : "success"}>{serverType(server.entry)}</Badge>
          {#if server.entry.enabled === false}<Badge tone="danger">disabled</Badge>{/if}
        </button>

        {#if expandedServers.has(server.name)}
          <div class="server__body">
            <div class="server__actions">
              <Button variant="ghost" size="sm" onclick={() => removeServer(server.name)}>
                <Trash2 size={14} />
                Remove server
              </Button>
            </div>

            {#if "type" in server.entry && server.entry.type === "remote"}
              <div class="server__fields">
                <div class="field">
                  <label for={`${server.name}-url`}>URL</label>
                  <TextInput
                    id={`${server.name}-url`}
                    value={server.entry.url ?? ""}
                    oninput={(event) => updateServer(server.name, (entry) => "type" in entry && entry.type === "remote" ? { ...entry, url: (event.currentTarget as HTMLInputElement).value } : entry)}
                    placeholder="https://mcp.example.com/mcp"
                  />
                </div>
                <div class="field field--inline">
                  <span class="field__label">Enabled</span>
                  <Toggle
                    checked={server.entry.enabled ?? true}
                    onCheckedChange={(checked) => updateServer(server.name, (entry) => ({ ...entry, enabled: checked }))}
                  />
                </div>
              </div>
            {:else if "type" in server.entry && server.entry.type === "local"}
              <div class="server__fields">
                <div class="field">
                  <label for={`${server.name}-command`}>Command (space-separated)</label>
                  <TextInput
                    id={`${server.name}-command`}
                    value={server.entry.command?.join(" ") ?? ""}
                    oninput={(event) => updateServer(server.name, (entry) => "type" in entry && entry.type === "local" ? { ...entry, command: (event.currentTarget as HTMLInputElement).value.split(" ").filter(Boolean) } : entry)}
                    placeholder="npx -y @modelcontextprotocol/server-filesystem"
                  />
                </div>
                <div class="field field--inline">
                  <span class="field__label">Enabled</span>
                  <Toggle
                    checked={server.entry.enabled ?? true}
                    onCheckedChange={(checked) => updateServer(server.name, (entry) => ({ ...entry, enabled: checked }))}
                  />
                </div>
              </div>
            {:else}
              <div class="field field--inline">
                <span class="field__label">Enabled</span>
                <Toggle
                  checked={server.entry.enabled}
                  onCheckedChange={(checked) => updateServer(server.name, () => ({ enabled: checked }))}
                />
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <div class="add-server">
    <div class="add-server__row">
      <TextInput
        bind:value={newServerName}
        placeholder="Server name (e.g. filesystem)"
        onkeydown={(event) => event.key === "Enter" && addServer()}
      />
      <div class="add-server__type" aria-label="Server type">
        <label>
          <input
            type="radio"
            name="serverType"
            value="remote"
            checked={newServerType === "remote"}
            onchange={() => newServerType = "remote"}
          />
          Remote
        </label>
        <label>
          <input
            type="radio"
            name="serverType"
            value="local"
            checked={newServerType === "local"}
            onchange={() => newServerType = "local"}
          />
          Local
        </label>
      </div>
      {#if newServerType === "remote"}
        <TextInput
          bind:value={newServerUrl}
          placeholder="https://mcp.example.com/mcp"
          onkeydown={(event) => event.key === "Enter" && addServer()}
        />
      {:else}
        <TextInput
          bind:value={newServerCommand}
          placeholder="npx -y @mcp/server"
          onkeydown={(event) => event.key === "Enter" && addServer()}
        />
      {/if}
      <Button variant="secondary" size="sm" onclick={addServer}>
        <Plus size={14} />
        Add server
      </Button>
    </div>
  </div>
</div>

<style>
  .mcp,
  .mcp__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .server {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .server__header {
    width: 100%;
    background: none;
    border: 0;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
  }

  .server__header:hover {
    background: var(--bg-2);
  }

  .server__toggle {
    color: var(--text-tertiary);
    display: flex;
  }

  .server__name {
    font-weight: 600;
    font-size: var(--text-sm);
    color: var(--text-primary);
    flex: 1;
  }

  .server__body {
    padding: var(--space-3);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .server__actions {
    display: flex;
    justify-content: flex-end;
  }

  .server__fields,
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .field--inline {
    flex-direction: row;
    align-items: center;
    gap: var(--space-2);
  }

  .field__label,
  .field label {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--text-tertiary);
  }

  .add-server {
    margin-top: var(--space-3);
    padding-top: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }

  .add-server__row {
    display: flex;
    gap: var(--space-2);
  }

  .add-server__row > :global(*) {
    flex: 1;
  }

  .add-server__type {
    flex: 0 0 auto;
    display: flex;
    gap: var(--space-2);
    align-items: center;
    font-size: var(--text-sm);
    color: var(--text-secondary);
  }

  .add-server__type label {
    display: flex;
    align-items: center;
    gap: 4px;
    cursor: pointer;
  }

  .add-server__type input[type="radio"] {
    accent-color: var(--accent);
  }
</style>
