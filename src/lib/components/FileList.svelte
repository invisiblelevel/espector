<script lang="ts">
  import type { IniFile } from "$lib/api";
  import { formatSize } from "$lib/utils";
  import { LL } from "$lib/i18n/state.svelte";
  import SearchBar from "./SearchBar.svelte";

  interface Props {
    files: IniFile[];
    selectedPath: string | null;
    search: string;
    onlyConflicts: boolean;
    onSearchChange: (v: string) => void;
    onOnlyConflictsChange: (v: boolean) => void;
    onSelect: (path: string) => void;
  }

  let {
    files,
    selectedPath,
    search,
    onlyConflicts,
    onSearchChange,
    onOnlyConflictsChange,
    onSelect,
  }: Props = $props();

  let L = $derived(LL());

  let filtered = $derived(
    files.filter((f) => {
      if (onlyConflicts && f.conflictingKeys.length === 0) return false;
      if (search.trim()) {
        const q = search.trim().toLowerCase();
        return f.relativePath.toLowerCase().includes(q) || f.name.toLowerCase().includes(q);
      }
      return true;
    }),
  );

  let headerText = $derived(
    filtered.length !== files.length
      ? `${L.fileList.filesCount} (${filtered.length} ${L.fileList.of} ${files.length})`
      : `${L.fileList.filesCount} (${files.length})`,
  );
</script>

<aside class="w-80 shrink-0 flex flex-col border-r border-[var(--color-border)] bg-[var(--color-bg-surface)]">
  <SearchBar
    value={search}
    {onlyConflicts}
    onValueChange={onSearchChange}
    onOnlyConflictsChange={onOnlyConflictsChange}
  />

  <div class="px-3 py-2 text-xs uppercase tracking-wide text-[var(--color-text-muted)] border-b border-[var(--color-border)]">
    {headerText}
  </div>

  <ul class="py-1 overflow-y-auto flex-1">
    {#each filtered as file (file.path)}
      <li>
        <button
          onclick={() => onSelect(file.path)}
          class="w-full text-left px-3 py-2 hover:bg-[var(--color-bg-hover)] transition-colors"
          class:bg-[var(--color-bg-elevated)]={selectedPath === file.path}
        >
          <div class="flex items-center gap-2">
            <span class="text-sm text-[var(--color-text-primary)] truncate flex-1" title={file.path}>
              {file.relativePath}
            </span>
            {#if file.conflictingKeys.length > 0}
              <span
                class="text-xs px-1.5 py-0.5 rounded bg-[var(--color-danger)]/20 text-[var(--color-danger)] shrink-0"
                title="{file.conflictingKeys.length} {L.toolbar.conflicts}"
              >
                {file.conflictingKeys.length}
              </span>
            {/if}
          </div>
          <div class="text-xs text-[var(--color-text-muted)] mt-0.5">
            {formatSize(file.size)}
          </div>
        </button>
      </li>
    {/each}

    {#if filtered.length === 0 && files.length > 0}
      <li class="px-3 py-4 text-sm text-[var(--color-text-muted)] text-center">
        {L.fileList.nothingFound}
      </li>
    {/if}
  </ul>
</aside>