<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    id: string;
    label?: string;
    helper?: string;
    error?: string;
    required?: boolean;
    children: Snippet;
  };

  let { id, label, helper, error, required, children }: Props = $props();
</script>

<div class="field" class:field--error={!!error}>
  {#if label}
    <label class="field__label" for={id}>
      {label}
      {#if required}<span class="field__required" aria-hidden="true">*</span>{/if}
    </label>
  {/if}
  {@render children()}
  {#if error}
    <p class="field__error" role="alert">{error}</p>
  {:else if helper}
    <p class="field__helper">{helper}</p>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field__label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-primary);
  }

  .field__required {
    color: var(--danger);
    margin-left: 2px;
  }

  .field__helper {
    margin: 0;
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }

  .field__error {
    margin: 0;
    font-size: var(--text-xs);
    color: var(--danger);
  }
</style>
