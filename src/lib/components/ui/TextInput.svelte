<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  type Props = HTMLInputAttributes & {
    value?: string;
    invalid?: boolean;
    mono?: boolean;
  };

  let {
    value = $bindable(""),
    invalid = false,
    mono = false,
    type = "text",
    class: className = "",
    ...rest
  }: Props = $props();
</script>

<input
  {type}
  bind:value
  class="input {className}"
  class:input--mono={mono}
  class:is-invalid={invalid}
  aria-invalid={invalid || undefined}
  {...rest}
/>

<style>
  .input {
    height: 32px;
    padding: 0 10px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    line-height: 1;
    width: 100%;
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .input--mono {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .input::placeholder {
    color: var(--text-tertiary);
  }

  .input:hover:not(:disabled):not(:focus) {
    border-color: var(--border-strong);
  }

  .input:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .input.is-invalid {
    border-color: var(--danger);
  }
  .input.is-invalid:focus {
    box-shadow: 0 0 0 2px var(--danger-muted);
  }

  .input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
