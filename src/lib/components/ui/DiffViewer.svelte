<script lang="ts">
  import { diffJson, type DiffLine } from "$lib/diff";

  type Props = {
    before: unknown;
    after: unknown;
    emptyLabel?: string;
  };

  let { before, after, emptyLabel = "No changes" }: Props = $props();

  const lines: DiffLine[] = $derived(diffJson(before, after));
  const hasChanges: boolean = $derived(lines.some((l) => l.kind !== "context"));
</script>

<div class="diff" role="region" aria-label="Diff">
  {#if !hasChanges}
    <p class="diff__empty">{emptyLabel}</p>
  {:else}
    <pre class="diff__pre"><code
        >{#each lines as line, i (`${i}-${line.kind}-${line.text}`)}<span
            class="diff__line diff__line--{line.kind}"
            ><span class="diff__prefix"
              >{line.kind === "add" ? "+" : line.kind === "remove" ? "-" : " "}</span
            ><span class="diff__text">{line.text}</span></span
          >{/each}</code
      ></pre>
  {/if}
</div>

<style>
  .diff {
    background: var(--bg-1);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .diff__empty {
    margin: 0;
    padding: var(--space-3) var(--space-4);
    color: var(--text-tertiary);
    font-size: var(--text-sm);
  }

  .diff__pre {
    margin: 0;
    padding: var(--space-2) 0;
    overflow-x: auto;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    line-height: 1.5;
  }

  .diff__line {
    display: block;
    padding: 0 var(--space-3);
    white-space: pre;
  }

  .diff__line--add {
    background: var(--diff-add-bg);
    border-left: 2px solid var(--diff-add-border);
  }
  .diff__line--remove {
    background: var(--diff-remove-bg);
    border-left: 2px solid var(--diff-remove-border);
  }
  .diff__line--context {
    border-left: 2px solid transparent;
    color: var(--text-secondary);
  }

  .diff__prefix {
    display: inline-block;
    width: 1.5ch;
    user-select: none;
    color: var(--text-tertiary);
  }

  .diff__line--add .diff__prefix {
    color: var(--success);
  }
  .diff__line--remove .diff__prefix {
    color: var(--danger);
  }
</style>
