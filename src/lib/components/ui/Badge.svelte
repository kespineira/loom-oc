<script lang="ts">
  import type { Snippet } from "svelte";

  type Tone = "neutral" | "success" | "warning" | "danger" | "info" | "accent";

  type Props = {
    tone?: Tone;
    children?: Snippet;
    class?: string;
  };

  let { tone = "neutral", children, class: className = "" }: Props = $props();
</script>

<span class="badge badge--{tone} {className}">
  {#if children}{@render children()}{/if}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    padding: 2px var(--space-2);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: 500;
    line-height: 1.4;
    text-transform: lowercase;
    letter-spacing: 0.01em;
  }

  .badge--neutral {
    background: var(--bg-3);
    color: var(--text-secondary);
  }
  .badge--success {
    background: var(--success-muted);
    color: var(--success);
  }
  .badge--warning {
    background: var(--warning-muted);
    color: var(--warning);
  }
  .badge--danger {
    background: var(--danger-muted);
    color: var(--danger);
  }
  .badge--info {
    background: var(--info-muted);
    color: var(--info);
  }
  .badge--accent {
    background: var(--accent-muted);
    color: var(--accent);
  }
</style>
