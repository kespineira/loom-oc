<script lang="ts">
  type Section = {
    id: string;
    label: string;
    description: string;
  };

  type Props = {
    active: string;
    dirty?: boolean;
    collapsed?: boolean;
    onSelect: (section: string) => void;
  };

  let { active, dirty = false, collapsed = false, onSelect }: Props = $props();

  const sections: Section[] = [
    { id: "settings", label: "Settings", description: "Shell, logs, server" },
    { id: "agents", label: "Agents", description: "Models and prompts" },
    { id: "providers", label: "Providers", description: "Custom model APIs" },
    { id: "mcp", label: "MCP Servers", description: "Local and remote tools" },
    { id: "permissions", label: "Permissions", description: "Tool access policy" },
    { id: "commands", label: "Commands", description: "Reusable prompts" },
    { id: "plugins", label: "Plugins", description: "Loaded extensions" },
  ];
</script>

<nav class="sidebar" class:is-collapsed={collapsed} aria-label="Configuration sections">
  <div class="sidebar__brand">
    <span class="sidebar__logo">L</span>
    {#if !collapsed}
      <div>
        <strong>Loom</strong>
        <span>OpenCode editor</span>
      </div>
    {/if}
  </div>

  <div class="sidebar__sections">
    {#each sections as section}
      <button
        type="button"
        class:active={active === section.id}
        onclick={() => onSelect(section.id)}
        title={collapsed ? section.label : undefined}
      >
        <span class="sidebar__dot" class:is-dirty={dirty && active === section.id}></span>
        {#if !collapsed}
          <span class="sidebar__label">{section.label}</span>
          <span class="sidebar__description">{section.description}</span>
        {/if}
      </button>
    {/each}
  </div>
</nav>

<style>
  .sidebar {
    width: 220px;
    min-width: 220px;
    height: 100vh;
    padding: var(--space-4) var(--space-3);
    background: var(--bg-1);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .sidebar.is-collapsed {
    width: 56px;
    min-width: 56px;
    padding-inline: var(--space-2);
  }

  .sidebar__brand {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: 32px;
  }

  .sidebar__logo {
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    background: var(--accent);
    color: var(--accent-text);
    font-weight: 700;
  }

  .sidebar__brand strong,
  .sidebar__brand span {
    display: block;
  }

  .sidebar__brand strong {
    font-size: var(--text-base);
    line-height: var(--leading-tight);
  }

  .sidebar__brand div > span {
    color: var(--text-tertiary);
    font-size: var(--text-xs);
  }

  .sidebar__sections {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sidebar__sections button {
    min-height: 42px;
    display: grid;
    grid-template-columns: 8px 1fr;
    column-gap: var(--space-2);
    align-items: center;
    padding: var(--space-2);
    background: transparent;
    border: 0;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    font-family: var(--font-sans);
    text-align: left;
  }

  .is-collapsed .sidebar__sections button {
    grid-template-columns: 1fr;
    justify-items: center;
  }

  .sidebar__sections button:hover,
  .sidebar__sections button.active {
    background: var(--bg-2);
    color: var(--text-primary);
  }

  .sidebar__sections button.active {
    box-shadow: inset 2px 0 0 var(--accent);
  }

  .sidebar__dot {
    width: 6px;
    height: 6px;
    border-radius: 999px;
    background: var(--border-strong);
  }

  .sidebar__dot.is-dirty {
    background: var(--accent);
  }

  .sidebar__label {
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .sidebar__description {
    grid-column: 2;
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }
</style>
