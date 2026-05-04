<script lang="ts">
  import { Badge, Button, TextInput } from "$lib/components/ui";
  import { ChevronDown, ChevronRight, Plus, Trash2, X } from "lucide-svelte";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  type ProviderEntry = {
    name: string;
    npm?: string;
    api?: string;
    displayName?: string;
    options?: {
      apiKey?: string;
      baseURL?: string;
      [key: string]: unknown;
    };
    models?: Record<string, Record<string, unknown>>;
  };

  let { config, onChange }: Props = $props();

  const providers: ProviderEntry[] = $derived(
    config.provider
      ? Object.entries(config.provider).map(([name, provider]) => {
          const raw = provider as Record<string, unknown>;
          return {
            name,
            npm: raw.npm as string | undefined,
            api: raw.api as string | undefined,
            displayName: raw.name as string | undefined,
            options: raw.options as ProviderEntry["options"],
            models: raw.models as ProviderEntry["models"],
          };
        })
      : [],
  );

  let expandedProviders = $state<Set<string>>(new Set());
  let expandedModels = $state<Set<string>>(new Set());
  let newProviderName = $state("");
  let newProviderNpm = $state("");
  let newProviderBaseUrl = $state("");
  let newProviderApiKey = $state("");
  let newModelName = $state("");
  let newModelDisplayName = $state("");
  let newModelId = $state("");

  function providerRecord(entries: ProviderEntry[]) {
    const result: Record<string, unknown> = {};
    for (const entry of entries) {
      const provider: Record<string, unknown> = {};
      if (entry.npm) provider.npm = entry.npm;
      if (entry.api) provider.api = entry.api;
      if (entry.displayName) provider.name = entry.displayName;
      if (entry.options && Object.keys(entry.options).length > 0) provider.options = entry.options;
      if (entry.models && Object.keys(entry.models).length > 0) provider.models = entry.models;
      result[entry.name] = provider;
    }
    return result;
  }

  function commit(entries: ProviderEntry[]) {
    const provider = providerRecord(entries);
    onChange({
      ...config,
      provider: Object.keys(provider).length ? provider as OpenCodeConfig["provider"] : undefined,
    });
  }

  function addProvider() {
    const name = newProviderName.trim().toLowerCase();
    if (!name || providers.some((provider) => provider.name === name)) {
      newProviderName = "";
      return;
    }

    const entry: ProviderEntry = { name };
    if (newProviderNpm.trim()) entry.npm = newProviderNpm.trim();
    if (newProviderBaseUrl.trim() || newProviderApiKey.trim()) {
      entry.options = {};
      if (newProviderBaseUrl.trim()) entry.options.baseURL = newProviderBaseUrl.trim();
      if (newProviderApiKey.trim()) entry.options.apiKey = newProviderApiKey.trim();
    }

    commit([...providers, entry]);
    expandedProviders = new Set([...expandedProviders, name]);
    newProviderName = "";
    newProviderNpm = "";
    newProviderBaseUrl = "";
    newProviderApiKey = "";
  }

  function removeProvider(name: string) {
    commit(providers.filter((provider) => provider.name !== name));
    expandedProviders = new Set([...expandedProviders].filter((key) => key !== name));
    expandedModels = new Set([...expandedModels].filter((key) => !key.startsWith(`${name}:`)));
  }

  function updateProviderField(name: string, field: string, value: string) {
    commit(
      providers.map((provider) => {
        if (provider.name !== name) return provider;
        if (field === "apiKey" || field === "baseURL") {
          const options = { ...(provider.options ?? {}) };
          if (value) options[field] = value;
          else delete options[field];
          return { ...provider, options: Object.keys(options).length ? options : undefined };
        }
        return { ...provider, [field]: value || undefined };
      }),
    );
  }

  function addModel(providerName: string) {
    const key = newModelName.trim();
    if (!key) return;

    commit(
      providers.map((provider) => {
        if (provider.name !== providerName || provider.models?.[key]) return provider;
        const model: Record<string, unknown> = {};
        if (newModelDisplayName.trim()) model.name = newModelDisplayName.trim();
        if (newModelId.trim()) model.id = newModelId.trim();
        return { ...provider, models: { ...(provider.models ?? {}), [key]: model } };
      }),
    );

    expandedModels = new Set([...expandedModels, `${providerName}:${key}`]);
    newModelName = "";
    newModelDisplayName = "";
    newModelId = "";
  }

  function removeModel(providerName: string, modelName: string) {
    commit(
      providers.map((provider) => {
        if (provider.name !== providerName || !provider.models) return provider;
        const { [modelName]: _removed, ...models } = provider.models;
        return { ...provider, models: Object.keys(models).length ? models : undefined };
      }),
    );
    expandedModels = new Set([...expandedModels].filter((key) => key !== `${providerName}:${modelName}`));
  }

  function updateModelField(providerName: string, modelName: string, field: string, value: string) {
    commit(
      providers.map((provider) => {
        if (provider.name !== providerName || !provider.models?.[modelName]) return provider;
        const model = { ...provider.models[modelName] };
        if (value) model[field] = value;
        else delete model[field];
        return { ...provider, models: { ...provider.models, [modelName]: model } };
      }),
    );
  }

  function toggleProvider(name: string) {
    const next = new Set(expandedProviders);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    expandedProviders = next;
  }

  function toggleModel(providerName: string, modelName: string) {
    const key = `${providerName}:${modelName}`;
    const next = new Set(expandedModels);
    if (next.has(key)) next.delete(key);
    else next.add(key);
    expandedModels = next;
  }
</script>

<div class="providers">
  <div class="providers__list">
    {#each providers as provider (provider.name)}
      <div class="provider">
        <button
          type="button"
          class="provider__header"
          aria-expanded={expandedProviders.has(provider.name)}
          onclick={() => toggleProvider(provider.name)}
        >
          <span class="provider__toggle" aria-hidden="true">
            {#if expandedProviders.has(provider.name)}
              <ChevronDown size={14} />
            {:else}
              <ChevronRight size={14} />
            {/if}
          </span>
          <span class="provider__name">{provider.name}</span>
          {#if provider.npm}<Badge tone="neutral">{provider.npm}</Badge>{/if}
          {#if provider.models && Object.keys(provider.models).length > 0}
            <Badge tone="success">
              {Object.keys(provider.models).length} model{Object.keys(provider.models).length !== 1 ? "s" : ""}
            </Badge>
          {/if}
        </button>

        {#if expandedProviders.has(provider.name)}
          <div class="provider__body">
            <div class="provider__actions">
              <Button variant="ghost" size="sm" onclick={() => removeProvider(provider.name)}>
                <Trash2 size={14} />
                Remove provider
              </Button>
            </div>

            <div class="provider__fields">
              <div class="field">
                <label for={`${provider.name}-npm`}>NPM Package</label>
                <TextInput
                  id={`${provider.name}-npm`}
                  value={provider.npm ?? ""}
                  oninput={(event) => updateProviderField(provider.name, "npm", (event.currentTarget as HTMLInputElement).value)}
                  placeholder="@ai-sdk/openai-compatible"
                />
              </div>
              <div class="field">
                <label for={`${provider.name}-display`}>Display Name</label>
                <TextInput
                  id={`${provider.name}-display`}
                  value={provider.displayName ?? ""}
                  oninput={(event) => updateProviderField(provider.name, "displayName", (event.currentTarget as HTMLInputElement).value)}
                  placeholder="LiteLLM"
                />
              </div>
              <div class="field">
                <label for={`${provider.name}-base-url`}>Base URL</label>
                <TextInput
                  id={`${provider.name}-base-url`}
                  value={provider.options?.baseURL ?? ""}
                  oninput={(event) => updateProviderField(provider.name, "baseURL", (event.currentTarget as HTMLInputElement).value)}
                  placeholder="https://api.example.com/v1"
                />
              </div>
              <div class="field">
                <label for={`${provider.name}-api-key`}>API Key</label>
                <TextInput
                  id={`${provider.name}-api-key`}
                  value={provider.options?.apiKey ?? ""}
                  oninput={(event) => updateProviderField(provider.name, "apiKey", (event.currentTarget as HTMLInputElement).value)}
                  placeholder="sk-..."
                  type="password"
                />
              </div>
            </div>

            <div class="models-section">
              <h4>Models</h4>
              {#if provider.models && Object.keys(provider.models).length > 0}
                {#each Object.entries(provider.models) as [modelName, model]}
                  <div class="model">
                    <button
                      type="button"
                      class="model__header"
                      aria-expanded={expandedModels.has(`${provider.name}:${modelName}`)}
                      onclick={() => toggleModel(provider.name, modelName)}
                    >
                      <span class="model__toggle" aria-hidden="true">
                        {#if expandedModels.has(`${provider.name}:${modelName}`)}
                          <ChevronDown size={12} />
                        {:else}
                          <ChevronRight size={12} />
                        {/if}
                      </span>
                      <span class="model__name">{modelName}</span>
                      {#if model.name}<span class="model__display">({model.name})</span>{/if}
                    </button>
                    {#if expandedModels.has(`${provider.name}:${modelName}`)}
                      <div class="model__body">
                        <div class="field">
                          <label for={`${provider.name}-${modelName}-id`}>Model ID</label>
                          <TextInput
                            id={`${provider.name}-${modelName}-id`}
                            value={(model.id as string | undefined) ?? ""}
                            oninput={(event) => updateModelField(provider.name, modelName, "id", (event.currentTarget as HTMLInputElement).value)}
                            placeholder="gpt-4"
                          />
                        </div>
                        <div class="field">
                          <label for={`${provider.name}-${modelName}-name`}>Display Name</label>
                          <TextInput
                            id={`${provider.name}-${modelName}-name`}
                            value={(model.name as string | undefined) ?? ""}
                            oninput={(event) => updateModelField(provider.name, modelName, "name", (event.currentTarget as HTMLInputElement).value)}
                            placeholder="GPT-4"
                          />
                        </div>
                        <Button variant="ghost" size="sm" onclick={() => removeModel(provider.name, modelName)}>
                          <X size={12} />
                          Remove model
                        </Button>
                      </div>
                    {/if}
                  </div>
                {/each}
              {:else}
                <p class="empty">No models configured for this provider.</p>
              {/if}

              <div class="add-model">
                <TextInput bind:value={newModelName} placeholder="Model key" />
                <TextInput bind:value={newModelDisplayName} placeholder="Display name" />
                <TextInput bind:value={newModelId} placeholder="Model ID (optional)" />
                <Button variant="secondary" size="sm" onclick={() => addModel(provider.name)}>
                  <Plus size={14} />
                  Add model
                </Button>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <div class="add-provider">
    <div class="add-provider__row">
      <TextInput
        bind:value={newProviderName}
        placeholder="Provider name (e.g. openai)"
        onkeydown={(event) => event.key === "Enter" && addProvider()}
      />
      <TextInput bind:value={newProviderNpm} placeholder="NPM package (optional)" />
      <Button variant="secondary" size="sm" onclick={addProvider}>
        <Plus size={14} />
        Add provider
      </Button>
    </div>
  </div>
</div>

<style>
  .providers,
  .providers__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .provider {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .provider__header,
  .model__header {
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
  }

  .provider__header {
    padding: var(--space-2) var(--space-3);
  }

  .provider__header:hover,
  .model__header:hover {
    background: var(--bg-2);
  }

  .provider__toggle,
  .model__toggle {
    color: var(--text-tertiary);
    display: flex;
  }

  .provider__name {
    font-weight: 600;
    font-size: var(--text-sm);
    color: var(--text-primary);
    flex: 1;
  }

  .provider__body {
    padding: var(--space-3);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .provider__actions {
    display: flex;
    justify-content: flex-end;
  }

  .provider__fields,
  .model__body {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-2);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field label {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--text-tertiary);
  }

  .models-section {
    padding-top: var(--space-3);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .models-section h4 {
    margin: 0;
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .model {
    border-radius: var(--radius-sm);
  }

  .model__header {
    padding: var(--space-1) var(--space-2);
  }

  .model__name {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--text-secondary);
  }

  .model__display,
  .empty {
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }

  .model__body {
    padding: var(--space-2);
  }

  .add-provider,
  .add-model {
    padding-top: var(--space-3);
    border-top: 1px solid var(--border-subtle);
  }

  .add-provider__row,
  .add-model {
    display: flex;
    gap: var(--space-2);
  }

  .add-provider__row > :global(*),
  .add-model > :global(*) {
    flex: 1;
  }
</style>
