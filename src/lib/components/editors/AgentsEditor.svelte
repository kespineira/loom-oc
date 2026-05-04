<script lang="ts">
  import { Badge, Button, NumberInput, Select, Textarea, TextInput, Toggle } from "$lib/components/ui";
  import { ChevronDown, ChevronRight, Plus, Trash2 } from "lucide-svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  type AgentEntry = Record<string, unknown> & {
    model?: string;
    variant?: string;
    description?: string;
    prompt?: string;
    mode?: "primary" | "subagent" | "all";
    disable?: boolean;
    hidden?: boolean;
    color?: string;
    steps?: number;
    temperature?: number;
    top_p?: number;
  };

  let { config, onChange }: Props = $props();

  const agents = $derived(
    config.agent
      ? Object.entries(config.agent as Record<string, AgentEntry>).map(([name, entry]) => ({ name, entry }))
      : [],
  );

  const agentNames = $derived(agents.map((agent) => agent.name));
  let expanded = $state<Set<string>>(new Set());
  let newAgentName = $state("");
  let newAgentMode = $state<"" | "primary" | "subagent" | "all">("");

  function commit(nextAgents: Record<string, AgentEntry>, nextDefaultAgent = config.default_agent) {
    onChange({
      ...config,
      agent: Object.keys(nextAgents).length ? nextAgents as OpenCodeConfig["agent"] : undefined,
      default_agent: nextDefaultAgent || undefined,
    });
  }

  function sanitize(entry: AgentEntry) {
    const next = { ...entry };
    for (const key of Object.keys(next)) {
      if (next[key] === "" || next[key] === undefined) delete next[key];
    }
    return next;
  }

  function addAgent() {
    const name = newAgentName.trim().toLowerCase();
    if (!name || agentNames.includes(name)) {
      newAgentName = "";
      return;
    }
    commit({
      ...((config.agent as Record<string, AgentEntry> | undefined) ?? {}),
      [name]: sanitize({ mode: newAgentMode || undefined }),
    });
    expanded = new Set([...expanded, name]);
    newAgentName = "";
    newAgentMode = "";
  }

  function removeAgent(name: string) {
    const { [name]: _removed, ...nextAgents } = (config.agent as Record<string, AgentEntry> | undefined) ?? {};
    const nextDefault = config.default_agent === name ? undefined : config.default_agent;
    commit(nextAgents, nextDefault);
    expanded = new Set([...expanded].filter((key) => key !== name));
  }

  function updateAgent(name: string, patch: Partial<AgentEntry>) {
    const current = ((config.agent as Record<string, AgentEntry> | undefined) ?? {})[name] ?? {};
    commit({
      ...((config.agent as Record<string, AgentEntry> | undefined) ?? {}),
      [name]: sanitize({ ...current, ...patch }),
    });
  }

  function toggle(name: string) {
    const next = new Set(expanded);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    expanded = next;
  }
</script>

<div class="agents">
  <div class="field">
    <label for="default-agent">Default agent</label>
    <Select
      id="default-agent"
      value={config.default_agent ?? ""}
      placeholder="No default agent"
      options={agentNames.map((name) => ({ value: name, label: name }))}
      onchange={(event) => commit((config.agent as Record<string, AgentEntry> | undefined) ?? {}, (event.currentTarget as HTMLSelectElement).value)}
    />
  </div>

  <div class="agents__list">
    {#each agents as agent (agent.name)}
      <div class="agent">
        <button type="button" class="agent__header" aria-expanded={expanded.has(agent.name)} onclick={() => toggle(agent.name)}>
          <span class="agent__toggle" aria-hidden="true">
            {#if expanded.has(agent.name)}<ChevronDown size={14} />{:else}<ChevronRight size={14} />{/if}
          </span>
          <span class="agent__name">{agent.name}</span>
          {#if agent.entry.mode}<Badge tone="neutral">{agent.entry.mode}</Badge>{/if}
          {#if agent.entry.disable}<Badge tone="danger">disabled</Badge>{/if}
        </button>

        {#if expanded.has(agent.name)}
          <div class="agent__body">
            <div class="agent__actions">
              <Button variant="ghost" size="sm" onclick={() => removeAgent(agent.name)}>
                <Trash2 size={14} />
                Remove agent
              </Button>
            </div>

            <div class="grid">
              <div class="field">
                <label for={`${agent.name}-model`}>Model</label>
                <TextInput id={`${agent.name}-model`} value={agent.entry.model ?? ""} oninput={(event) => updateAgent(agent.name, { model: (event.currentTarget as HTMLInputElement).value })} placeholder="anthropic/claude-sonnet-4-5" />
              </div>
              <div class="field">
                <label for={`${agent.name}-variant`}>Variant</label>
                <TextInput id={`${agent.name}-variant`} value={agent.entry.variant ?? ""} oninput={(event) => updateAgent(agent.name, { variant: (event.currentTarget as HTMLInputElement).value })} placeholder="thinking" />
              </div>
              <div class="field">
                <label for={`${agent.name}-mode`}>Mode</label>
                <Select
                  id={`${agent.name}-mode`}
                  value={agent.entry.mode ?? ""}
                  placeholder="Unset"
                  options={[{ value: "primary", label: "primary" }, { value: "subagent", label: "subagent" }, { value: "all", label: "all" }]}
                  onchange={(event) => updateAgent(agent.name, { mode: (event.currentTarget as HTMLSelectElement).value as AgentEntry["mode"] })}
                />
              </div>
              <div class="field">
                <label for={`${agent.name}-color`}>Color</label>
                <TextInput id={`${agent.name}-color`} value={agent.entry.color ?? ""} oninput={(event) => updateAgent(agent.name, { color: (event.currentTarget as HTMLInputElement).value })} placeholder="blue or #4f46e5" />
              </div>
              <div class="field">
                <label for={`${agent.name}-steps`}>Steps</label>
                <NumberInput id={`${agent.name}-steps`} value={agent.entry.steps ?? null} min={1} onValueChange={(value) => updateAgent(agent.name, { steps: value ?? undefined })} />
              </div>
              <div class="field">
                <label for={`${agent.name}-temperature`}>Temperature</label>
                <NumberInput id={`${agent.name}-temperature`} value={agent.entry.temperature ?? null} min={0} max={2} step={0.1} onValueChange={(value) => updateAgent(agent.name, { temperature: value ?? undefined })} />
              </div>
              <div class="field field--inline">
                <span class="field__label">Disabled</span>
                <Toggle checked={agent.entry.disable ?? false} onCheckedChange={(checked) => updateAgent(agent.name, { disable: checked })} />
              </div>
              <div class="field field--inline">
                <span class="field__label">Hidden</span>
                <Toggle checked={agent.entry.hidden ?? false} onCheckedChange={(checked) => updateAgent(agent.name, { hidden: checked })} />
              </div>
            </div>

            <div class="field">
              <label for={`${agent.name}-description`}>Description</label>
              <TextInput id={`${agent.name}-description`} value={agent.entry.description ?? ""} oninput={(event) => updateAgent(agent.name, { description: (event.currentTarget as HTMLInputElement).value })} />
            </div>
            <div class="field">
              <label for={`${agent.name}-prompt`}>Prompt</label>
              <Textarea id={`${agent.name}-prompt`} value={agent.entry.prompt ?? ""} mono rows={8} oninput={(event) => updateAgent(agent.name, { prompt: (event.currentTarget as HTMLTextAreaElement).value })} placeholder="Agent system prompt" />
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <div class="add-agent">
    <TextInput bind:value={newAgentName} placeholder="Agent name" onkeydown={(event) => event.key === "Enter" && addAgent()} />
    <Select bind:value={newAgentMode} placeholder="Mode" options={[{ value: "primary", label: "primary" }, { value: "subagent", label: "subagent" }, { value: "all", label: "all" }]} />
    <Button variant="secondary" size="sm" onclick={addAgent}><Plus size={14} />Add agent</Button>
  </div>
</div>

<style>
  .agents,
  .agents__list,
  .agent__body {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .agent {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .agent__header {
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

  .agent__header:hover {
    background: var(--bg-2);
  }

  .agent__toggle {
    display: flex;
    color: var(--text-tertiary);
  }

  .agent__name {
    flex: 1;
    color: var(--text-primary);
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .agent__body {
    padding: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }

  .agent__actions {
    display: flex;
    justify-content: flex-end;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-2);
  }

  .field {
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

  .add-agent {
    display: flex;
    gap: var(--space-2);
    padding-top: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }

  .add-agent > :global(*) {
    flex: 1;
  }
</style>
