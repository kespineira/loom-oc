<script lang="ts">
  import { X, CheckCircle2, AlertTriangle, AlertOctagon, Info } from "lucide-svelte";
  import { toasts, type ToastTone } from "$lib/stores/toast.svelte";

  const ICONS: Record<ToastTone, typeof CheckCircle2> = {
    success: CheckCircle2,
    warning: AlertTriangle,
    danger: AlertOctagon,
    info: Info,
  };
</script>

<div class="toaster" aria-live="polite" aria-atomic="false">
  {#each toasts.items as toast (toast.id)}
    {@const Icon = ICONS[toast.tone]}
    <div class="toast toast--{toast.tone}" role="status">
      <Icon size={16} class="toast__icon" />
      <div class="toast__body">
        <p class="toast__title">{toast.title}</p>
        {#if toast.description}
          <p class="toast__description">{toast.description}</p>
        {/if}
      </div>
      <button
        type="button"
        class="toast__close"
        aria-label="Dismiss"
        onclick={() => toasts.dismiss(toast.id)}
      >
        <X size={14} />
      </button>
    </div>
  {/each}
</div>

<style>
  .toaster {
    position: fixed;
    bottom: var(--space-5);
    right: var(--space-5);
    display: flex;
    flex-direction: column-reverse;
    gap: var(--space-2);
    pointer-events: none;
    z-index: 100;
  }

  .toast {
    pointer-events: auto;
    width: 360px;
    padding: var(--space-3) var(--space-4);
    background: var(--bg-2);
    border: 1px solid var(--border-default);
    border-left-width: 3px;
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    animation: toast-in var(--duration-normal) var(--ease);
  }

  .toast--success {
    border-left-color: var(--success);
  }
  .toast--warning {
    border-left-color: var(--warning);
  }
  .toast--danger {
    border-left-color: var(--danger);
  }
  .toast--info {
    border-left-color: var(--info);
  }

  :global(.toast__icon) {
    flex-shrink: 0;
    margin-top: 2px;
  }
  .toast--success :global(.toast__icon) {
    color: var(--success);
  }
  .toast--warning :global(.toast__icon) {
    color: var(--warning);
  }
  .toast--danger :global(.toast__icon) {
    color: var(--danger);
  }
  .toast--info :global(.toast__icon) {
    color: var(--info);
  }

  .toast__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toast__title {
    margin: 0;
    font-size: var(--text-base);
    font-weight: 500;
    color: var(--text-primary);
    line-height: var(--leading-tight);
  }

  .toast__description {
    margin: 0;
    font-size: var(--text-sm);
    color: var(--text-secondary);
    line-height: var(--leading-normal);
  }

  .toast__close {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    background: transparent;
    border: 0;
    border-radius: var(--radius-sm);
    color: var(--text-tertiary);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .toast__close:hover {
    background: var(--bg-3);
    color: var(--text-primary);
  }

  @keyframes toast-in {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: none;
    }
  }
</style>
