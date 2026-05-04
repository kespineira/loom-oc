<script lang="ts">
  import { Button, TextInput } from "$lib/components/ui";
  import { Plus, X } from "lucide-svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  let { config, onChange }: Props = $props();

  const plugins: string[] = $derived((config.plugin ?? []) as string[]);
  let newPlugin = $state("");

  function commit(nextPlugins: string[]) {
    onChange({ ...config, plugin: nextPlugins.length ? nextPlugins : undefined });
  }

  function addPlugin() {
    const value = newPlugin.trim();
    if (!value || plugins.includes(value)) {
      newPlugin = "";
      return;
    }
    commit([...plugins, value]);
    newPlugin = "";
  }

  function removePlugin(index: number) {
    commit(plugins.filter((_, i) => i !== index));
  }
</script>

<div class="plugins">
  {#if plugins.length === 0}
    <p class="empty">No plugins configured.</p>
  {:else}
    <ul class="plugins__list">
      {#each plugins as plugin, i (plugin)}
        <li class="plugins__item">
          <span class="plugins__name">{plugin}</span>
          <Button variant="ghost" size="sm" onclick={() => removePlugin(i)}>
            <X size={14} />
            Remove
          </Button>
        </li>
      {/each}
    </ul>
  {/if}

  <div class="add-plugin">
    <TextInput
      bind:value={newPlugin}
      placeholder="Plugin name or path"
      onkeydown={(event) => event.key === "Enter" && addPlugin()}
    />
    <Button variant="secondary" size="sm" onclick={addPlugin}>
      <Plus size={14} />
      Add plugin
    </Button>
  </div>
</div>

<style>
  .plugins {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .empty {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .plugins__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .plugins__item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
  }

  .plugins__name {
    flex: 1;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--text-secondary);
  }

  .add-plugin {
    display: flex;
    gap: var(--space-2);
  }

  .add-plugin > :global(*) {
    flex: 1;
  }
</style>
