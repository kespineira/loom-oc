<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { Loader2 } from "lucide-svelte";

  type Variant = "primary" | "secondary" | "ghost" | "danger";
  type Size = "sm" | "md" | "lg";

  type Props = HTMLButtonAttributes & {
    variant?: Variant;
    size?: Size;
    loading?: boolean;
    iconLeft?: Snippet;
    iconRight?: Snippet;
    children?: Snippet;
  };

  let {
    variant = "secondary",
    size = "md",
    loading = false,
    disabled,
    type = "button",
    iconLeft,
    iconRight,
    children,
    class: className = "",
    ...rest
  }: Props = $props();
</script>

<button
  {type}
  class="btn btn--{variant} btn--{size} {className}"
  class:is-loading={loading}
  disabled={disabled || loading}
  {...rest}
>
  {#if loading}
    <span class="icon" aria-hidden="true">
      <Loader2 size={size === "sm" ? 14 : 16} class="spin" />
    </span>
  {:else if iconLeft}
    <span class="icon" aria-hidden="true">{@render iconLeft()}</span>
  {/if}
  {#if children}<span class="label">{@render children()}</span>{/if}
  {#if iconRight && !loading}
    <span class="icon" aria-hidden="true">{@render iconRight()}</span>
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-weight: 500;
    line-height: 1;
    white-space: nowrap;
    cursor: pointer;
    transition:
      background-color var(--duration-fast) var(--ease),
      border-color var(--duration-fast) var(--ease),
      color var(--duration-fast) var(--ease);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.is-loading {
    cursor: progress;
  }

  /* Sizes */
  .btn--sm {
    height: 28px;
    padding: 0 var(--space-3);
    font-size: var(--text-sm);
  }
  .btn--md {
    height: 32px;
    padding: 0 var(--space-4);
    font-size: var(--text-base);
  }
  .btn--lg {
    height: 40px;
    padding: 0 var(--space-4);
    font-size: var(--text-md);
  }

  /* Variants */
  .btn--primary {
    background: var(--accent);
    color: var(--accent-text);
  }
  .btn--primary:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .btn--secondary {
    background: var(--bg-2);
    color: var(--text-primary);
    border-color: var(--border-default);
  }
  .btn--secondary:hover:not(:disabled) {
    background: var(--bg-3);
    border-color: var(--border-strong);
  }

  .btn--ghost {
    background: transparent;
    color: var(--text-secondary);
  }
  .btn--ghost:hover:not(:disabled) {
    background: var(--bg-2);
    color: var(--text-primary);
  }

  .btn--danger {
    background: var(--danger-muted);
    color: var(--danger);
    border-color: var(--danger);
  }
  .btn--danger:hover:not(:disabled) {
    background: var(--danger);
    color: var(--accent-text);
  }

  .icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
