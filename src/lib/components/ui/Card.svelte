<script lang="ts">
  import type { Snippet } from "svelte";

  type Props = {
    title?: string;
    description?: string;
    actions?: Snippet;
    children?: Snippet;
    tone?: "default" | "danger";
    class?: string;
  };

  let {
    title,
    description,
    actions,
    children,
    tone = "default",
    class: className = "",
  }: Props = $props();
</script>

<section class="card card--{tone} {className}">
  {#if title || actions}
    <header class="card__header">
      <div class="card__heading">
        {#if title}<h2 class="card__title">{title}</h2>{/if}
        {#if description}<p class="card__description">{description}</p>{/if}
      </div>
      {#if actions}<div class="card__actions">{@render actions()}</div>{/if}
    </header>
  {/if}
  {#if children}<div class="card__body">{@render children()}</div>{/if}
</section>

<style>
  .card {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: var(--space-4) var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .card--danger {
    border-color: color-mix(in oklab, var(--danger) 50%, var(--border-subtle));
  }

  .card__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .card__heading {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
  }

  .card__title {
    margin: 0;
    font-size: var(--text-lg);
    font-weight: 600;
    color: var(--text-primary);
    line-height: var(--leading-tight);
  }

  .card__description {
    margin: 0;
    font-size: var(--text-sm);
    color: var(--text-secondary);
  }

  .card__actions {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-shrink: 0;
  }

  .card__body {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
</style>
