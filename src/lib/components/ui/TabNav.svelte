<script lang="ts">
  let { active, onTabChange }: { active: string; onTabChange: (tab: string) => void } = $props();

  function handleTabClick(tabId: string) {
    onTabChange(tabId);
  }

  const tabs = [
    { id: "settings", label: "Settings" },
    { id: "agents", label: "Agents" },
    { id: "providers", label: "Providers" },
    { id: "mcp", label: "MCP Servers" },
    { id: "permissions", label: "Permissions" },
    { id: "commands", label: "Commands" },
    { id: "plugins", label: "Plugins" },
  ];
</script>

<div class="tabs" role="tablist" aria-label="Config sections">
  {#each tabs as tab}
    <button
      role="tab"
      aria-selected={active === tab.id}
      class:active={active === tab.id}
      onclick={() => handleTabClick(tab.id)}
    >
      {tab.label}
    </button>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    gap: 2px;
    border-bottom: 1px solid var(--border-subtle);
    margin: 0;
    padding: 0;
  }

  .tabs button {
    background: none;
    border: none;
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-tertiary);
    cursor: pointer;
    border-bottom: 2px solid transparent;
    transition: color 150ms, border-color 150ms;
    font-family: inherit;
  }

  .tabs button:hover {
    color: var(--text-secondary);
  }

  .tabs button.active {
    color: var(--text-primary);
    border-bottom-color: var(--accent);
  }
</style>
