<script lang="ts">
  import type { ConflictGroup } from "$lib/api";
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    conflicts: ConflictGroup[];
    onFileClick: (path: string) => void;
  }

  let { conflicts, onFileClick }: Props = $props();

  let L = $derived(LL());

  let sorted = $derived(
    [...conflicts].sort((a, b) => {
      if (a.valuesDiffer !== b.valuesDiffer) return a.valuesDiffer ? -1 : 1;
      return a.fullKey.localeCompare(b.fullKey);
    }),
  );

  let differingCount = $derived(conflicts.filter((c) => c.valuesDiffer).length);
  let sameCount = $derived(conflicts.length - differingCount);

  let subtitle = $derived(
    `${differingCount} ${L.conflictList.differingCount}, ${sameCount} ${L.conflictList.sameCount}`,
  );
</script>

<section class="flex-1 overflow-y-auto bg-[var(--color-bg-base)]">
  <div class="px-4 py-3 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)]">
    <div class="text-sm font-medium text-[var(--color-text-primary)]">
      {L.conflictList.title} ({conflicts.length})
    </div>
    <div class="text-xs text-[var(--color-text-muted)] mt-0.5">
      {subtitle}
    </div>
  </div>

  {#if conflicts.length === 0}
    <div class="flex items-center justify-center h-64 text-sm text-[var(--color-text-muted)]">
      {L.conflictList.noConflicts}
    </div>
  {:else}
    <div class="p-4 space-y-3">
      {#each sorted as conflict (conflict.fullKey)}
        <div class="rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-surface)] overflow-hidden">
          <div class="px-4 py-2 border-b border-[var(--color-border)] flex items-center justify-between gap-3">
            <div class="flex items-center gap-2 min-w-0">
              <span class="text-xs text-[var(--color-text-muted)] shrink-0">[{conflict.section}]</span>
              <span class="text-sm font-medium text-[var(--color-text-primary)] truncate">
                {conflict.key}
              </span>
              {#if conflict.valuesDiffer && conflict.uniqueValuesCount > 1}
                <span class="text-xs text-[var(--color-text-muted)] shrink-0">
                  {conflict.uniqueValuesCount} {L.conflictList.uniqueValues}
                </span>
              {/if}
            </div>
            {#if conflict.valuesDiffer}
              <span class="text-xs px-2 py-0.5 rounded bg-[var(--color-danger)]/20 text-[var(--color-danger)] shrink-0">
                {L.conflictList.valuesDiffer}
              </span>
            {:else}
              <span class="text-xs px-2 py-0.5 rounded bg-[var(--color-warning)]/20 text-[var(--color-warning)] shrink-0">
                {L.conflictList.valuesSame}
              </span>
            {/if}
          </div>

          <ul>
            {#each conflict.occurrences as occ (occ.filePath + ":" + occ.line)}
              {@const isWinner = occ.filePath === conflict.winner.filePath}
              <li>
                <button
                  onclick={() => onFileClick(occ.filePath)}
                                    class="w-full text-left px-4 py-2 hover:bg-[var(--color-bg-hover)] transition-colors flex items-center gap-3 {isWinner ? 'bg-[var(--color-success)]/5' : ''}"
                >
                  <span class="text-xs text-[var(--color-text-secondary)] truncate flex-1" title={occ.filePath}>
                    {occ.fileRelativePath}
                  </span>

                  {#if isWinner}
                    <span class="text-xs px-2 py-0.5 rounded bg-[var(--color-success)]/20 text-[var(--color-success)] shrink-0">
                      {L.conflictList.winner}
                    </span>
                  {/if}

                  <span class="text-xs text-[var(--color-text-muted)] shrink-0">
                    {L.conflictList.page} {occ.line}
                  </span>
                  <span class="text-sm text-[var(--color-text-primary)] font-mono shrink-0">
                    = {occ.value}
                  </span>
                </button>
              </li>
            {/each}
          </ul>
        </div>
      {/each}
    </div>
  {/if}
</section>