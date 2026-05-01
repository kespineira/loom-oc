<script lang="ts">
  type Props = {
    id?: string;
    checked?: boolean;
    disabled?: boolean;
    label?: string;
    "aria-label"?: string;
  };

  let {
    id,
    checked = $bindable(false),
    disabled = false,
    label,
    "aria-label": ariaLabel,
  }: Props = $props();

  function toggle() {
    if (!disabled) checked = !checked;
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === " " || e.key === "Enter") {
      e.preventDefault();
      toggle();
    }
  }
</script>

<button
  {id}
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={ariaLabel ?? label}
  {disabled}
  class="toggle"
  class:is-on={checked}
  onclick={toggle}
  {onkeydown}
>
  <span class="toggle__thumb"></span>
</button>

<style>
  .toggle {
    position: relative;
    width: 28px;
    height: 16px;
    border-radius: 999px;
    background: var(--bg-3);
    border: 1px solid var(--border-default);
    cursor: pointer;
    padding: 0;
    transition: background-color var(--duration-fast) var(--ease);
  }

  .toggle:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle.is-on {
    background: var(--accent);
    border-color: var(--accent);
  }

  .toggle__thumb {
    position: absolute;
    top: 1px;
    left: 1px;
    width: 12px;
    height: 12px;
    border-radius: 999px;
    background: var(--text-primary);
    transition: transform var(--duration-fast) var(--ease);
  }

  .toggle.is-on .toggle__thumb {
    background: var(--accent-text);
    transform: translateX(12px);
  }
</style>
