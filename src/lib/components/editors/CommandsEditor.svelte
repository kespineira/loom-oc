<script lang="ts">
  import { Badge, Button, Select, Textarea, TextInput, Toggle } from "$lib/components/ui";
  import { ChevronDown, ChevronRight, Plus, Trash2 } from "lucide-svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  type CommandEntry = {
    template: string;
    description?: string;
    agent?: string;
    model?: string;
    subtask?: boolean;
  };

  let { config, onChange }: Props = $props();

  const commands = $derived(
    config.command
      ? Object.entries(config.command as Record<string, CommandEntry>).map(([name, entry]) => ({ name, entry }))
      : [],
  );
  const agentOptions = $derived(
    Object.keys((config.agent as Record<string, unknown> | undefined) ?? {}).map((name) => ({ value: name, label: name })),
  );

  let expanded = $state<Set<string>>(new Set());
  let newCommandName = $state("");
  let newCommandTemplate = $state("");

  function clean(entry: CommandEntry): CommandEntry {
    return {
      template: entry.template,
      description: entry.description || undefined,
      agent: entry.agent || undefined,
      model: entry.model || undefined,
      subtask: entry.subtask || undefined,
    };
  }

  function commit(nextCommands: Record<string, CommandEntry>) {
    onChange({
      ...config,
      command: Object.keys(nextCommands).length ? nextCommands as OpenCodeConfig["command"] : undefined,
    });
  }

  function addCommand() {
    const name = newCommandName.trim();
    const template = newCommandTemplate.trim();
    if (!name || !template || commands.some((command) => command.name === name)) return;
    commit({ ...((config.command as Record<string, CommandEntry> | undefined) ?? {}), [name]: { template } });
    expanded = new Set([...expanded, name]);
    newCommandName = "";
    newCommandTemplate = "";
  }

  function removeCommand(name: string) {
    const { [name]: _removed, ...next } = (config.command as Record<string, CommandEntry> | undefined) ?? {};
    commit(next);
    expanded = new Set([...expanded].filter((key) => key !== name));
  }

  function updateCommand(name: string, patch: Partial<CommandEntry>) {
    const current = ((config.command as Record<string, CommandEntry> | undefined) ?? {})[name];
    if (!current) return;
    commit({
      ...((config.command as Record<string, CommandEntry> | undefined) ?? {}),
      [name]: clean({ ...current, ...patch }),
    });
  }

  function toggle(name: string) {
    const next = new Set(expanded);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    expanded = next;
  }
</script>

<div class="commands">
  {#if commands.length === 0}
    <p class="empty">No commands configured.</p>
  {:else}
    <div class="commands__list">
      {#each commands as command (command.name)}
        <div class="command">
          <button type="button" class="command__header" aria-expanded={expanded.has(command.name)} onclick={() => toggle(command.name)}>
            <span class="command__toggle" aria-hidden="true">
              {#if expanded.has(command.name)}<ChevronDown size={14} />{:else}<ChevronRight size={14} />{/if}
            </span>
            <span class="command__name">/{command.name}</span>
            {#if command.entry.agent}<Badge tone="neutral">{command.entry.agent}</Badge>{/if}
            {#if command.entry.subtask}<Badge tone="success">subtask</Badge>{/if}
          </button>

          {#if expanded.has(command.name)}
            <div class="command__body">
              <div class="command__actions">
                <Button variant="ghost" size="sm" onclick={() => removeCommand(command.name)}>
                  <Trash2 size={14} />
                  Remove command
                </Button>
              </div>

              <div class="field">
                <label for={`${command.name}-template`}>Template</label>
                <Textarea id={`${command.name}-template`} value={command.entry.template} mono rows={8} oninput={(event) => updateCommand(command.name, { template: (event.currentTarget as HTMLTextAreaElement).value })} />
              </div>

              <div class="grid">
                <div class="field">
                  <label for={`${command.name}-description`}>Description</label>
                  <TextInput id={`${command.name}-description`} value={command.entry.description ?? ""} oninput={(event) => updateCommand(command.name, { description: (event.currentTarget as HTMLInputElement).value })} />
                </div>
                <div class="field">
                  <label for={`${command.name}-agent`}>Agent</label>
                  <Select id={`${command.name}-agent`} value={command.entry.agent ?? ""} placeholder="No agent" options={agentOptions} onchange={(event) => updateCommand(command.name, { agent: (event.currentTarget as HTMLSelectElement).value })} />
                </div>
                <div class="field">
                  <label for={`${command.name}-model`}>Model override</label>
                  <TextInput id={`${command.name}-model`} value={command.entry.model ?? ""} oninput={(event) => updateCommand(command.name, { model: (event.currentTarget as HTMLInputElement).value })} placeholder="provider/model" />
                </div>
                <div class="field field--inline">
                  <span class="field__label">Run as subtask</span>
                  <Toggle checked={command.entry.subtask ?? false} onCheckedChange={(checked) => updateCommand(command.name, { subtask: checked })} />
                </div>
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  <div class="add-command">
    <TextInput bind:value={newCommandName} placeholder="Command name" />
    <Textarea bind:value={newCommandTemplate} rows={3} placeholder="Command template" />
    <Button variant="secondary" size="sm" onclick={addCommand}><Plus size={14} />Add command</Button>
  </div>
</div>

<style>
  .commands,
  .commands__list,
  .command__body {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .command {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .command__header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: none;
    border: 0;
    color: inherit;
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .command__header:hover {
    background: var(--bg-2);
  }

  .command__toggle {
    display: flex;
    color: var(--text-tertiary);
  }

  .command__name {
    flex: 1;
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .command__body {
    padding: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }

  .command__actions {
    display: flex;
    justify-content: flex-end;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-2);
  }

  .field,
  .add-command {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field--inline {
    flex-direction: row;
    align-items: center;
    gap: var(--space-2);
  }

  .field label,
  .field__label {
    color: var(--text-tertiary);
    font-size: var(--text-xs);
    font-weight: 500;
  }

  .empty {
    margin: 0;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .add-command {
    padding-top: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }
</style>
