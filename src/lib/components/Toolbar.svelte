<script lang="ts">
  import { pickFolder } from "$lib/api";
  import { LL, changeLocale, SUPPORTED_LOCALES, locale } from "$lib/i18n/state.svelte";
  import type { Locale } from "$lib/i18n/state.svelte";

  interface Props {
    rootPath: string;
    totalFiles: number;
    totalKeys: number;
    conflictCount: number;
    loading: boolean;
    onFolderPicked: (path: string) => void;
    onExportClick: () => void;
    onMergeClick: () => void;
    onInfoClick: () => void;
  }

  let {
    rootPath,
    totalFiles,
    totalKeys,
    conflictCount,
    loading,
    onFolderPicked,
    onExportClick,
    onMergeClick,
    onInfoClick,
  }: Props = $props();

  let L = $derived(LL());

  async function handlePick() {
    const path = await pickFolder();
    if (path) onFolderPicked(path);
  }

  function handleLocaleChange(e: Event) {
    const value = (e.currentTarget as HTMLSelectElement).value as Locale;
    changeLocale(value);
  }
</script>

<header class="flex items-center gap-4 px-4 py-3 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)]">
  <button
    onclick={handlePick}
    disabled={loading}
    class="px-4 py-2 rounded-md bg-[var(--color-accent)] hover:bg-[var(--color-accent-hover)] text-white font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed shrink-0"
  >
    {loading ? L.toolbar.scanning : L.toolbar.pickFolder}
  </button>

  {#if rootPath}
    <div class="flex items-center gap-2 shrink-0">
      <button
        onclick={onExportClick}
        class="px-3 py-2 rounded-md text-sm font-medium transition-colors
               bg-[var(--color-bg-base)] text-[var(--color-text-secondary)]
               border border-[var(--color-border)]
               hover:border-[var(--color-accent)] hover:text-[var(--color-accent)]"
      >
        {L.toolbar.exportReport}
      </button>

      <button
        onclick={onMergeClick}
        class="px-3 py-2 rounded-md text-sm font-medium transition-colors
               bg-[var(--color-bg-base)] text-[var(--color-text-secondary)]
               border border-[var(--color-border)]
               hover:border-[var(--color-accent)] hover:text-[var(--color-accent)]"
      >
        {L.toolbar.mergeIni}
      </button>
    </div>

    <div class="flex-1 min-w-0">
      <div class="text-xs text-[var(--color-text-muted)]">{L.toolbar.folder}</div>
      <div class="text-sm text-[var(--color-text-secondary)] truncate" title={rootPath}>
        {rootPath}
      </div>
    </div>

    <div class="flex items-center gap-4 text-sm shrink-0">
      <div>
        <span class="text-[var(--color-text-muted)]">{L.toolbar.files}: </span>
        <span class="text-[var(--color-text-primary)] font-medium">{totalFiles}</span>
      </div>
      <div>
        <span class="text-[var(--color-text-muted)]">{L.toolbar.keys}: </span>
        <span class="text-[var(--color-text-primary)] font-medium">{totalKeys}</span>
      </div>
      <div>
        <span class="text-[var(--color-text-muted)]">{L.toolbar.conflicts}: </span>
        <span
          class="font-medium"
          class:text-[var(--color-danger)]={conflictCount > 0}
          class:text-[var(--color-text-primary)]={conflictCount === 0}
        >
          {conflictCount}
        </span>
      </div>
    </div>
  {/if}

  <div class="flex items-center gap-2 shrink-0 ml-auto">
    <select
      value={locale.value}
      onchange={handleLocaleChange}
      class="px-2 py-1 rounded-md text-xs cursor-pointer
             bg-[var(--color-bg-base)] text-[var(--color-text-secondary)]
             border border-[var(--color-border)]
             hover:border-[var(--color-accent)] focus:outline-none
             focus:border-[var(--color-accent)] transition-colors"
      aria-label={L.toolbar.language}
    >
      {#each SUPPORTED_LOCALES as l (l)}
        <option value={l}>{l.toUpperCase()}</option>
      {/each}
    </select>

    <button
      onclick={onInfoClick}
      title={L.toolbar.info}
      aria-label={L.toolbar.info}
      class="w-8 h-8 rounded-md flex items-center justify-center text-sm font-semibold
             bg-[var(--color-bg-base)] text-[var(--color-text-secondary)]
             border border-[var(--color-border)]
             hover:border-[var(--color-accent)] hover:text-[var(--color-accent)]
             transition-colors cursor-pointer"
    >
      i
    </button>
  </div>
</header>