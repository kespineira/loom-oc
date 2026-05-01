<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { FolderOpen } from "lucide-svelte";

  type Props = {
    id?: string;
    value?: string;
    placeholder?: string;
    directory?: boolean;
    invalid?: boolean;
    disabled?: boolean;
  };

  let {
    id,
    value = $bindable(""),
    placeholder,
    directory = false,
    invalid = false,
    disabled = false,
  }: Props = $props();

  async function browse() {
    const selected = await open({
      directory,
      multiple: false,
      defaultPath: value || undefined,
    });
    if (typeof selected === "string") value = selected;
  }
</script>

<div class="path" class:is-invalid={invalid}>
  <input
    {id}
    type="text"
    {placeholder}
    {disabled}
    bind:value
    aria-invalid={invalid || undefined}
    class="path__input"
  />
  <button
    type="button"
    class="path__browse"
    onclick={browse}
    {disabled}
    aria-label="Browse"
  >
    <FolderOpen size={14} />
    <span>Browse</span>
  </button>
</div>

<style>
  .path {
    display: flex;
    align-items: stretch;
    height: 32px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .path:hover:not(:focus-within) {
    border-color: var(--border-strong);
  }

  .path:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .path.is-invalid {
    border-color: var(--danger);
  }

  .path__input {
    flex: 1;
    min-width: 0;
    padding: 0 10px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    outline: none;
  }

  .path__input::placeholder {
    color: var(--text-tertiary);
  }

  .path__browse {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    padding: 0 var(--space-3);
    background: var(--bg-2);
    border: 0;
    border-left: 1px solid var(--border-default);
    color: var(--text-secondary);
    font-size: var(--text-sm);
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--duration-fast) var(--ease);
  }

  .path__browse:hover:not(:disabled) {
    background: var(--bg-3);
    color: var(--text-primary);
  }

  .path__browse:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
