<script lang="ts">
  import { Check, X, HelpCircle } from "lucide-svelte";

  export type PermissionState = "allow" | "ask" | "deny";

  type Row = { id: string; label: string };
  type Column = { id: string; label: string };

  type Props = {
    rows: Row[];
    columns: Column[];
    value: Record<string, Record<string, PermissionState>>;
    onChange?: (next: Record<string, Record<string, PermissionState>>) => void;
  };

  let { rows, columns, value = $bindable(), onChange }: Props = $props();

  const ORDER: PermissionState[] = ["allow", "ask", "deny"];

  function next(state: PermissionState): PermissionState {
    return ORDER[(ORDER.indexOf(state) + 1) % ORDER.length];
  }

  function setCell(rowId: string, colId: string, state: PermissionState) {
    const updated = {
      ...value,
      [rowId]: { ...(value[rowId] ?? {}), [colId]: state },
    };
    value = updated;
    onChange?.(updated);
  }

  function setColumn(colId: string, state: PermissionState) {
    const updated: typeof value = { ...value };
    for (const r of rows) {
      updated[r.id] = { ...(updated[r.id] ?? {}), [colId]: state };
    }
    value = updated;
    onChange?.(updated);
  }

  function onCellClick(e: MouseEvent, rowId: string, colId: string) {
    const current = value[rowId]?.[colId] ?? "ask";
    const newState = next(current);
    if (e.shiftKey) {
      setColumn(colId, newState);
    } else {
      setCell(rowId, colId, newState);
    }
  }

  function stateOf(rowId: string, colId: string): PermissionState {
    return value[rowId]?.[colId] ?? "ask";
  }
</script>

<div class="matrix" role="grid">
  <div class="matrix__scroll">
    <table>
      <thead>
        <tr>
          <th class="matrix__corner" scope="col"></th>
          {#each columns as col (col.id)}
            <th scope="col" class="matrix__col-head">
              <span class="matrix__col-label">{col.label}</span>
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each rows as row (row.id)}
          <tr>
            <th scope="row" class="matrix__row-head">{row.label}</th>
            {#each columns as col (col.id)}
              {@const state = stateOf(row.id, col.id)}
              <td class="matrix__cell">
                <button
                  type="button"
                  class="cell cell--{state}"
                  aria-label="{row.label} / {col.label}: {state}"
                  onclick={(e) => onCellClick(e, row.id, col.id)}
                >
                  {#if state === "allow"}
                    <Check size={14} />
                  {:else if state === "ask"}
                    <HelpCircle size={14} />
                  {:else}
                    <X size={14} />
                  {/if}
                </button>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .matrix {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .matrix__scroll {
    overflow: auto;
    max-height: 60vh;
  }

  table {
    border-collapse: separate;
    border-spacing: 0;
    width: max-content;
    min-width: 100%;
  }

  th,
  td {
    padding: 0;
    margin: 0;
    border-right: 1px solid var(--border-subtle);
    border-bottom: 1px solid var(--border-subtle);
    text-align: left;
  }

  thead th {
    position: sticky;
    top: 0;
    background: var(--bg-2);
    z-index: 2;
    height: 32px;
    padding: 0 var(--space-3);
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .matrix__col-head {
    text-align: center;
  }

  .matrix__col-label {
    font-family: var(--font-mono);
  }

  .matrix__row-head {
    position: sticky;
    left: 0;
    background: var(--bg-1);
    z-index: 1;
    height: 32px;
    padding: 0 var(--space-3);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
  }

  .matrix__corner {
    left: 0;
    z-index: 3;
    background: var(--bg-2);
    min-width: 160px;
  }

  .matrix__cell {
    width: 32px;
    height: 32px;
    text-align: center;
  }

  .cell {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: 0;
    cursor: pointer;
    color: var(--text-secondary);
    transition: background-color var(--duration-fast) var(--ease);
  }

  .cell--allow {
    background: var(--success-muted);
    color: var(--success);
  }
  .cell--ask {
    background: var(--warning-muted);
    color: var(--warning);
  }
  .cell--deny {
    background: var(--danger-muted);
    color: var(--danger);
  }

  .cell:hover {
    filter: brightness(1.15);
  }
</style>
