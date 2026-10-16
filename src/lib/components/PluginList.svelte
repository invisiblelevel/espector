<script lang="ts">
  import type { PluginFile } from "$lib/api";
  import { formatSize } from "$lib/utils";
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    plugins: PluginFile[];
    selectedPath: string | null;
    onSelect: (path: string) => void;
  }

  let { plugins, selectedPath, onSelect }: Props = $props();

  let L = $derived(LL());

  function kindBadgeClass(kind: string): string {
    if (kind === "esl") return "bg-[var(--color-accent)]/20 text-[var(--color-accent)]";
    if (kind === "esm") return "bg-[var(--color-warning)]/20 text-[var(--color-warning)]";
    return "bg-[var(--color-bg-hover)] text-[var(--color-text-muted)]";
  }
</script>

<aside class="w-80 shrink-0 flex flex-col border-r border-[var(--color-border)] bg-[var(--color-bg-surface)]">
  <div class="px-3 py-2 text-xs uppercase tracking-wide text-[var(--color-text-muted)] border-b border-[var(--color-border)]">
    {L.plugins.listTitle} ({plugins.length})
  </div>

  <ul class="py-1 overflow-y-auto flex-1">
    {#each plugins as plugin (plugin.path)}
      <li>
        <button
          onclick={() => onSelect(plugin.path)}
          class="w-full text-left px-3 py-2 transition-all duration-150
                 hover:bg-[var(--color-bg-hover)]
                 {selectedPath === plugin.path ? 'bg-[var(--color-bg-elevated)] border-l-2 border-l-[var(--color-accent)]' : 'border-l-2 border-l-transparent'}"
        >
          <div class="flex items-center gap-2">
            <span class="text-sm text-[var(--color-text-primary)] truncate flex-1" title={plugin.path}>
              {plugin.name}
            </span>
            <span class="text-[10px] px-1.5 py-0.5 rounded uppercase font-semibold shrink-0 {kindBadgeClass(plugin.kind)}">
              {plugin.kind}
            </span>
          </div>
          <div class="text-xs text-[var(--color-text-muted)] mt-0.5">
            {formatSize(plugin.size)} · {plugin.totalRecords} {L.plugins.recordsShort}
          </div>
        </button>
      </li>
    {/each}
  </ul>
</aside>