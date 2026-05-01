<script lang="ts" generics="T extends string | number">
  import { ChevronDown } from "lucide-svelte";

  type Option = { value: T; label: string };

  type Props = {
    id?: string;
    value?: T | null;
    options: Option[];
    placeholder?: string;
    invalid?: boolean;
    disabled?: boolean;
  };

  let {
    id,
    value = $bindable<T | null>(null),
    options,
    placeholder = "Select…",
    invalid = false,
    disabled = false,
  }: Props = $props();
</script>

<div class="sel" class:is-invalid={invalid} class:is-disabled={disabled}>
  <select
    {id}
    bind:value
    {disabled}
    aria-invalid={invalid || undefined}
    class="sel__input"
  >
    {#if placeholder}
      <option value="" disabled selected={value === null || value === undefined}>
        {placeholder}
      </option>
    {/if}
    {#each options as opt (opt.value)}
      <option value={opt.value}>{opt.label}</option>
    {/each}
  </select>
  <ChevronDown size={14} class="sel__chevron" aria-hidden="true" />
</div>

<style>
  .sel {
    position: relative;
    height: 32px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .sel:hover:not(:focus-within):not(.is-disabled) {
    border-color: var(--border-strong);
  }

  .sel:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .sel.is-invalid {
    border-color: var(--danger);
  }

  .sel.is-disabled {
    opacity: 0.5;
  }

  .sel__input {
    width: 100%;
    height: 100%;
    padding: 0 28px 0 10px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    appearance: none;
    cursor: pointer;
    outline: none;
  }

  .sel__input:disabled {
    cursor: not-allowed;
  }

  :global(.sel__chevron) {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-tertiary);
    pointer-events: none;
  }

  /* Dropdown options inherit OS styling — Tauri renders native pop-up.
     We can't reliably style <option> across platforms. */
</style>
