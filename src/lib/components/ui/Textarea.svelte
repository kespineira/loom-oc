<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";

  type Props = HTMLTextareaAttributes & {
    value?: string;
    invalid?: boolean;
    mono?: boolean;
    minRows?: number;
    maxRows?: number;
  };

  let {
    value = $bindable(""),
    invalid = false,
    mono = false,
    minRows = 6,
    maxRows = 24,
    rows,
    class: className = "",
    ...rest
  }: Props = $props();
</script>

<textarea
  bind:value
  rows={rows ?? minRows}
  class="ta {className}"
  class:ta--mono={mono}
  class:is-invalid={invalid}
  aria-invalid={invalid || undefined}
  style:--ta-min-rows={minRows}
  style:--ta-max-rows={maxRows}
  {...rest}
></textarea>

<style>
  .ta {
    width: 100%;
    padding: var(--space-2) 10px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    line-height: var(--leading-normal);
    resize: vertical;
    min-height: calc(var(--ta-min-rows, 6) * 1.5em + var(--space-4));
    max-height: calc(var(--ta-max-rows, 24) * 1.5em + var(--space-4));
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .ta--mono {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .ta::placeholder {
    color: var(--text-tertiary);
  }

  .ta:hover:not(:focus):not(:disabled) {
    border-color: var(--border-strong);
  }

  .ta:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .ta.is-invalid {
    border-color: var(--danger);
  }

  .ta:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
