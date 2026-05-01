<script lang="ts" generics="T extends string">
  import { ChevronDown, Check } from "lucide-svelte";

  type Option = { value: T; label: string; hint?: string };

  type Props = {
    id?: string;
    value?: T | null;
    options: Option[];
    placeholder?: string;
    invalid?: boolean;
    disabled?: boolean;
    mono?: boolean;
  };

  let {
    id,
    value = $bindable<T | null>(null),
    options,
    placeholder = "Search…",
    invalid = false,
    disabled = false,
    mono = false,
  }: Props = $props();

  let open = $state(false);
  let query = $state("");
  let activeIndex = $state(0);
  let inputEl = $state<HTMLInputElement>();
  let listEl = $state<HTMLUListElement>();

  const selectedLabel = $derived(
    value !== null && value !== undefined
      ? (options.find((o) => o.value === value)?.label ?? String(value))
      : "",
  );

  const filtered = $derived.by(() => {
    if (!query.trim()) return options;
    const q = query.toLowerCase();
    return options.filter(
      (o) => o.label.toLowerCase().includes(q) || String(o.value).toLowerCase().includes(q),
    );
  });

  function openMenu() {
    if (disabled) return;
    open = true;
    query = "";
    activeIndex = Math.max(
      0,
      filtered.findIndex((o) => o.value === value),
    );
    queueMicrotask(() => inputEl?.focus());
  }

  function closeMenu() {
    open = false;
    query = "";
  }

  function pick(opt: Option) {
    value = opt.value;
    closeMenu();
  }

  function onkeydown(e: KeyboardEvent) {
    if (!open) {
      if (e.key === "ArrowDown" || e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        openMenu();
      }
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      activeIndex = Math.min(filtered.length - 1, activeIndex + 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeIndex = Math.max(0, activeIndex - 1);
    } else if (e.key === "Enter") {
      e.preventDefault();
      const opt = filtered[activeIndex];
      if (opt) pick(opt);
    } else if (e.key === "Escape") {
      e.preventDefault();
      closeMenu();
    }
  }

  function onBlurContainer(e: FocusEvent) {
    const next = e.relatedTarget as Node | null;
    if (next && (e.currentTarget as Node).contains(next)) return;
    closeMenu();
  }
</script>

<div
  class="cb"
  class:is-open={open}
  class:is-invalid={invalid}
  class:is-disabled={disabled}
  onfocusout={onBlurContainer}
  {onkeydown}
  role="presentation"
>
  <button
    {id}
    type="button"
    class="cb__trigger"
    class:cb__trigger--mono={mono}
    {disabled}
    aria-haspopup="listbox"
    aria-expanded={open}
    onclick={() => (open ? closeMenu() : openMenu())}
  >
    <span class="cb__value" class:cb__value--placeholder={!selectedLabel}>
      {selectedLabel || placeholder}
    </span>
    <ChevronDown size={14} class="cb__chevron" />
  </button>

  {#if open}
    <div class="cb__panel">
      <input
        bind:this={inputEl}
        bind:value={query}
        type="text"
        class="cb__search"
        class:cb__search--mono={mono}
        {placeholder}
        oninput={() => (activeIndex = 0)}
      />
      <ul bind:this={listEl} class="cb__list" role="listbox">
        {#each filtered as opt, i (opt.value)}
          <li>
            <button
              type="button"
              role="option"
              aria-selected={value === opt.value}
              class="cb__option"
              class:cb__option--active={i === activeIndex}
              class:cb__option--mono={mono}
              onmousemove={() => (activeIndex = i)}
              onclick={() => pick(opt)}
            >
              <span class="cb__option-label">{opt.label}</span>
              {#if opt.hint}<span class="cb__option-hint">{opt.hint}</span>{/if}
              {#if value === opt.value}
                <Check size={14} class="cb__option-check" />
              {/if}
            </button>
          </li>
        {:else}
          <li class="cb__empty">No matches</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  .cb {
    position: relative;
  }

  .cb__trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
    width: 100%;
    height: 32px;
    padding: 0 10px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    cursor: pointer;
    text-align: left;
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .cb__trigger--mono {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .cb:hover:not(.is-open):not(.is-disabled) .cb__trigger {
    border-color: var(--border-strong);
  }

  .cb.is-open .cb__trigger,
  .cb__trigger:focus-visible {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
    outline: none;
  }

  .cb.is-invalid .cb__trigger {
    border-color: var(--danger);
  }

  .cb.is-disabled .cb__trigger {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .cb__value {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .cb__value--placeholder {
    color: var(--text-tertiary);
  }

  :global(.cb__chevron) {
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .cb__panel {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    background: var(--bg-2);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 50;
    display: flex;
    flex-direction: column;
    max-height: 320px;
    overflow: hidden;
    animation: cb-in var(--duration-normal) var(--ease);
  }

  @keyframes cb-in {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: none;
    }
  }

  .cb__search {
    height: 32px;
    padding: 0 10px;
    background: var(--bg-1);
    border: 0;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    outline: none;
  }

  .cb__search--mono {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .cb__list {
    list-style: none;
    margin: 0;
    padding: var(--space-1) 0;
    overflow-y: auto;
    max-height: 280px;
  }

  .cb__option {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    height: 28px;
    padding: 0 10px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--text-base);
    text-align: left;
    cursor: pointer;
  }

  .cb__option--mono .cb__option-label {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .cb__option--active {
    background: var(--bg-3);
  }

  .cb__option-label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .cb__option-hint {
    color: var(--text-tertiary);
    font-size: var(--text-xs);
  }

  :global(.cb__option-check) {
    color: var(--accent);
  }

  .cb__empty {
    padding: var(--space-2) 10px;
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }
</style>
