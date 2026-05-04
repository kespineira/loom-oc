<script lang="ts">
  import { Minus, Plus } from "lucide-svelte";

  type Props = {
    id?: string;
    value?: number | null;
    min?: number;
    max?: number;
    step?: number;
    placeholder?: string;
    invalid?: boolean;
    disabled?: boolean;
    onValueChange?: (value: number | null) => void;
  };

  let {
    id,
    value = $bindable<number | null>(null),
    min,
    max,
    step = 1,
    placeholder,
    invalid = false,
    disabled = false,
    onValueChange,
  }: Props = $props();

  function clamp(n: number): number {
    if (min !== undefined && n < min) n = min;
    if (max !== undefined && n > max) n = max;
    return n;
  }

  function bump(direction: 1 | -1) {
    const current = value ?? 0;
    value = clamp(current + step * direction);
    onValueChange?.(value);
  }

  function handleInput() {
    onValueChange?.(value);
  }
</script>

<div class="num" class:is-invalid={invalid}>
  <input
    {id}
    type="number"
    {min}
    {max}
    {step}
    {placeholder}
    {disabled}
    bind:value
    oninput={handleInput}
    aria-invalid={invalid || undefined}
    class="num__input"
  />
  <div class="num__steppers">
    <button
      type="button"
      class="num__stepper"
      onclick={() => bump(-1)}
      {disabled}
      tabindex="-1"
      aria-label="Decrement"
    >
      <Minus size={12} />
    </button>
    <button
      type="button"
      class="num__stepper"
      onclick={() => bump(1)}
      {disabled}
      tabindex="-1"
      aria-label="Increment"
    >
      <Plus size={12} />
    </button>
  </div>
</div>

<style>
  .num {
    position: relative;
    height: 32px;
    background: var(--bg-1);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    display: flex;
    align-items: stretch;
    transition:
      border-color var(--duration-fast) var(--ease),
      box-shadow var(--duration-fast) var(--ease);
  }

  .num:hover:not(:focus-within) {
    border-color: var(--border-strong);
  }

  .num:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .num.is-invalid {
    border-color: var(--danger);
  }

  .num__input {
    flex: 1;
    min-width: 0;
    padding: 0 10px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-variant-numeric: tabular-nums;
    outline: none;
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .num__input::-webkit-outer-spin-button,
  .num__input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .num__steppers {
    display: flex;
    flex-direction: column;
    border-left: 1px solid var(--border-default);
    opacity: 0;
    transition: opacity var(--duration-fast) var(--ease);
  }

  .num:hover .num__steppers,
  .num:focus-within .num__steppers {
    opacity: 1;
  }

  .num__stepper {
    flex: 1;
    width: 22px;
    background: var(--bg-2);
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .num__stepper:first-child {
    border-bottom: 1px solid var(--border-default);
    border-top-right-radius: calc(var(--radius-md) - 1px);
  }

  .num__stepper:last-child {
    border-bottom-right-radius: calc(var(--radius-md) - 1px);
  }

  .num__stepper:hover:not(:disabled) {
    background: var(--bg-3);
    color: var(--text-primary);
  }

  .num__stepper:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
