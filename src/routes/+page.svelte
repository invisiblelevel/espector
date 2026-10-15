<script lang="ts">
  import {
    scanFolder,
    pickSavePath,
    pickSaveIniPath,
    saveReport,
    buildMergedIni,
    type ScanResult,
    type IniFile,
  } from "$lib/api";
  import { LL, locale } from "$lib/i18n/state.svelte";
  import { buildMarkdownReport } from "$lib/report";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import TabSwitcher from "$lib/components/TabSwitcher.svelte";
  import FileList from "$lib/components/FileList.svelte";
  import IniViewer from "$lib/components/IniViewer.svelte";
  import ConflictList from "$lib/components/ConflictList.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import AboutModal from "$lib/components/AboutModal.svelte";
  import SupportModal from "$lib/components/SupportModal.svelte";

  let result: ScanResult | null = $state(null);
  let selectedPath: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);

  let mode: "files" | "conflicts" = $state("files");
  let search = $state("");
  let onlyConflicts = $state(false);
  let modal: "none" | "about" | "support" = $state("none");

  let L = $derived(LL());

  let selectedFile: IniFile | null = $derived(
    result && selectedPath
      ? (result.files.find((f) => f.path === selectedPath) ?? null)
      : null,
  );

  async function handleFolderPicked(path: string) {
    loading = true;
    error = null;
    selectedPath = null;
    search = "";
    onlyConflicts = false;
    mode = "files";
    try {
      result = await scanFolder(path);
      if (result.files.length > 0) {
        selectedPath = result.files[0].path;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      result = null;
    } finally {
      loading = false;
    }
  }

  function handleConflictFileClick(path: string) {
    selectedPath = path;
    mode = "files";
  }

  async function handleExportClick() {
    if (!result) return;
    try {
      const markdown = buildMarkdownReport(result, locale.value, L);
      const path = await pickSavePath("espector-report.md");
      if (!path) return;
      await saveReport(path, markdown);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  async function handleMergeClick() {
    if (!result) return;
    try {
      const text = await buildMergedIni(result);
      const path = await pickSaveIniPath("merged.ini");
      if (!path) return;
      await saveReport(path, text);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

<div class="h-screen flex flex-col">
  <Toolbar
    rootPath={result?.rootPath ?? ""}
    totalFiles={result?.files.length ?? 0}
    totalKeys={result?.totalKeys ?? 0}
    conflictCount={result?.conflictCount ?? 0}
    {loading}
    onFolderPicked={handleFolderPicked}
    onExportClick={handleExportClick}
    onMergeClick={handleMergeClick}
    onInfoClick={() => (modal = "about")}
  />

  {#if error}
    <div class="px-4 py-3 bg-[var(--color-danger)]/15 border-b border-[var(--color-danger)]/30 text-sm text-[var(--color-danger)]">
      {L.errors.prefix} {error}
    </div>
  {/if}

  {#if result && result.files.length > 0}
    <TabSwitcher
      {mode}
      filesCount={result.files.length}
      conflictsCount={result.conflicts.length}
      onChange={(m) => (mode = m)}
    />
  {/if}

  <main class="flex-1 flex min-h-0">
    {#if result && result.files.length > 0}
      {#if mode === "files"}
        <FileList
          files={result.files}
          {selectedPath}
          {search}
          {onlyConflicts}
          onSearchChange={(v) => (search = v)}
          onOnlyConflictsChange={(v) => (onlyConflicts = v)}
          onSelect={(path) => (selectedPath = path)}
        />
        {#if selectedFile}
          <IniViewer file={selectedFile} conflicts={result.conflicts} />
        {:else}
          <EmptyState hasRoot={true} hasSelection={false} />
        {/if}
      {:else}
        <ConflictList conflicts={result.conflicts} onFileClick={handleConflictFileClick} />
      {/if}
    {:else}
      <EmptyState hasRoot={!!result} hasSelection={false} />
    {/if}
  </main>
</div>

{#if modal === "about"}
  <AboutModal
    onClose={() => (modal = "none")}
    onOpenSupport={() => (modal = "support")}
  />
{:else if modal === "support"}
  <SupportModal onClose={() => (modal = "none")} />
{/if}