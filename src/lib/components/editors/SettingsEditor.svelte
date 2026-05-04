<script lang="ts">
  import { NumberInput, Select, Textarea, TextInput, Toggle } from "$lib/components/ui";
  import type { OpenCodeConfig } from "$lib/schema/opencode";

  type Props = {
    config: OpenCodeConfig;
    onChange: (config: OpenCodeConfig) => void;
  };

  let { config, onChange }: Props = $props();

  const corsInput = $derived((config.server?.cors ?? []).join(", "));

  function update(next: Partial<OpenCodeConfig>) {
    onChange({ ...config, ...next });
  }

  function updateServer(next: NonNullable<OpenCodeConfig["server"]>) {
    const server = { ...(config.server ?? {}), ...next };
    for (const key of Object.keys(server) as Array<keyof typeof server>) {
      if (server[key] === undefined) delete server[key];
    }
    update({ server: Object.keys(server).length ? server : undefined });
  }

  function parseCors(value: string) {
    return value
      .split(",")
      .map((item) => item.trim())
      .filter(Boolean);
  }
</script>

<div class="settings">
  <div class="field">
    <label for="shell">Shell</label>
    <TextInput
      id="shell"
      value={config.shell ?? ""}
      placeholder="/bin/zsh"
      oninput={(event) => update({ shell: (event.currentTarget as HTMLInputElement).value || undefined })}
    />
  </div>

  <div class="field">
    <label for="logLevel">Log Level</label>
    <Select
      id="logLevel"
      value={config.logLevel ?? "INFO"}
      options={[
        { value: "DEBUG", label: "DEBUG" },
        { value: "INFO", label: "INFO" },
        { value: "WARN", label: "WARN" },
        { value: "ERROR", label: "ERROR" },
      ]}
      onchange={(event) => update({ logLevel: (event.currentTarget as HTMLSelectElement).value as OpenCodeConfig["logLevel"] })}
    />
  </div>

  <div class="section">
    <h3>Server</h3>

    <div class="field">
      <label for="serverPort">Port</label>
      <NumberInput
        id="serverPort"
        value={config.server?.port ?? null}
        min={1}
        max={65535}
        onValueChange={(value) => updateServer({ port: value || undefined })}
      />
    </div>

    <div class="field">
      <label for="serverHostname">Hostname</label>
      <TextInput
        id="serverHostname"
        value={config.server?.hostname ?? ""}
        placeholder="localhost"
        oninput={(event) => updateServer({ hostname: (event.currentTarget as HTMLInputElement).value || undefined })}
      />
    </div>

    <div class="field field--inline">
      <span class="field__label">Enable mDNS</span>
      <Toggle
        checked={config.server?.mdns ?? false}
        onCheckedChange={(checked) => updateServer({ mdns: checked })}
      />
    </div>

    {#if config.server?.mdns}
      <div class="field">
        <label for="mdnsDomain">mDNS Domain</label>
        <TextInput
          id="mdnsDomain"
          value={config.server?.mdnsDomain ?? ""}
          placeholder="opencode.local"
          oninput={(event) => updateServer({ mdnsDomain: (event.currentTarget as HTMLInputElement).value || undefined })}
        />
      </div>
    {/if}

    <div class="field">
      <label for="serverCors">CORS Domains (comma-separated)</label>
      <Textarea
        id="serverCors"
        value={corsInput}
        oninput={(event) => updateServer({ cors: parseCors((event.currentTarget as HTMLTextAreaElement).value) })}
        placeholder="http://localhost:5173, https://example.com"
        rows={2}
      />
    </div>
  </div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
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

  .field__label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-secondary);
  }

  .field label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-secondary);
  }

  .section {
    border-top: 1px solid var(--border-subtle);
    padding-top: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .section h3 {
    margin: 0;
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--text-primary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
</style>
