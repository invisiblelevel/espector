<script lang="ts">
  import type { PluginFile } from "$lib/api";
  import { formatSize } from "$lib/utils";
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    plugin: PluginFile;
  }

  let { plugin }: Props = $props();

  let L = $derived(LL());

  // Сортируем типы по убыванию количества — самые частые сверху.
  let sortedRecords = $derived(
    [...plugin.recordCounts].sort((a, b) => b.count - a.count),
  );

  // Максимум для полосок пропорций.
  let maxCount = $derived(
    plugin.recordCounts.length > 0
      ? Math.max(...plugin.recordCounts.map((r) => r.count))
      : 1,
  );
</script>

<section class="flex-1 overflow-y-auto bg-[var(--color-bg-base)] animate-fade-in">
  <div class="sticky top-0 px-4 py-2 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)] z-10">
    <div class="text-sm font-medium text-[var(--color-text-primary)]">{plugin.name}</div>
    <div class="text-xs text-[var(--color-text-muted)] truncate" title={plugin.path}>{plugin.path}</div>
  </div>

  <div class="p-6 space-y-6">
    <!-- Флаги -->
    <div class="flex flex-wrap gap-2">
      {#if plugin.header.isEsm}
        <span class="text-xs px-2 py-1 rounded bg-[var(--color-warning)]/20 text-[var(--color-warning)] font-semibold">ESM</span>
      {/if}
      {#if plugin.header.isEsl}
        <span class="text-xs px-2 py-1 rounded bg-[var(--color-accent)]/20 text-[var(--color-accent)] font-semibold">ESL</span>
      {/if}
      {#if plugin.header.isLocalized}
        <span class="text-xs px-2 py-1 rounded bg-[var(--color-bg-hover)] text-[var(--color-text-secondary)] font-semibold">Localized</span>
      {/if}
      <span class="text-xs px-2 py-1 rounded bg-[var(--color-bg-hover)] text-[var(--color-text-secondary)] font-semibold uppercase">{plugin.kind}</span>
    </div>

    <!-- Основные данные -->
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-1">{L.plugins.author}</div>
        <div class="text-[var(--color-text-primary)]">{plugin.header.author ?? "—"}</div>
      </div>
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-1">{L.plugins.version}</div>
        <div class="text-[var(--color-text-primary)]">{plugin.header.version.toFixed(2)}</div>
      </div>
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-1">{L.plugins.size}</div>
        <div class="text-[var(--color-text-primary)]">{formatSize(plugin.size)}</div>
      </div>
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-1">{L.plugins.records}</div>
        <div class="text-[var(--color-text-primary)]">{plugin.totalRecords}</div>
      </div>
    </div>

    <!-- Описание -->
    {#if plugin.header.description}
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-1">{L.plugins.description}</div>
        <p class="text-sm text-[var(--color-text-secondary)] whitespace-pre-line leading-relaxed">
          {plugin.header.description}
        </p>
      </div>
    {/if}

    <!-- Записи по типам -->
    {#if plugin.recordCounts.length > 0}
      <div>
        <div class="text-xs text-[var(--color-text-muted)] mb-3">
          {L.plugins.recordTypes} ({plugin.recordCounts.length})
        </div>
        <div class="space-y-1.5">
          {#each sortedRecords as rec (rec.recordType)}
            <div class="flex items-center gap-3">
              <span class="text-xs font-mono text-[var(--color-text-secondary)] w-16 shrink-0">
                {rec.recordType}
              </span>
              <div class="flex-1 h-1.5 rounded bg-[var(--color-bg-surface)] overflow-hidden">
                <div
                  class="h-full bg-[var(--color-accent)] rounded"
                  style="width: {(rec.count / maxCount) * 100}%"
                ></div>
              </div>
              <span class="text-xs text-[var(--color-text-primary)] font-medium w-10 text-right shrink-0">
                {rec.count}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Мастера -->
    <div>
      <div class="text-xs text-[var(--color-text-muted)] mb-2">
        {L.plugins.masters} ({plugin.header.masters.length})
      </div>
      {#if plugin.header.masters.length === 0}
        <p class="text-sm text-[var(--color-text-muted)] italic">{L.plugins.noMasters}</p>
      {:else}
        <ul class="space-y-1">
          {#each plugin.header.masters as master (master)}
            <li class="text-sm font-mono text-[var(--color-text-secondary)] px-2 py-1 rounded bg-[var(--color-bg-surface)] border border-[var(--color-border)]">
              {master}
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  </div>
</section>