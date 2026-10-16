<script lang="ts">
  import {
    scanFolder,
    pickSavePath,
    pickSaveIniPath,
    saveReport,
    buildMergedIni,
    type ScanResult,
    type IniFile,
    type PluginFile,
  } from "$lib/api";
  import { LL, locale } from "$lib/i18n/state.svelte";
  import { buildMarkdownReport, buildCsvReport } from "$lib/report";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import TabSwitcher from "$lib/components/TabSwitcher.svelte";
  import FileList from "$lib/components/FileList.svelte";
  import IniViewer from "$lib/components/IniViewer.svelte";
  import ConflictList from "$lib/components/ConflictList.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import PluginList from "$lib/components/PluginList.svelte";
  import PluginViewer from "$lib/components/PluginViewer.svelte";
  import AboutModal from "$lib/components/AboutModal.svelte";
  import SupportModal from "$lib/components/SupportModal.svelte";
  import LoadingSkeleton from "$lib/components/LoadingSkeleton.svelte";

  type Mode = "files" | "conflicts" | "plugins";

  let result: ScanResult | null = $state(null);
  let selectedIniPath: string | null = $state(null);
  let selectedPluginPath: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);

  let mode: Mode = $state("files");
  let search = $state("");
  let onlyConflicts = $state(false);
  let modal: "none" | "about" | "support" = $state("none");

  let L = $derived(LL());

  let selectedFile: IniFile | null = $derived(
    result && selectedIniPath
      ? (result.files.find((f) => f.path === selectedIniPath) ?? null)
      : null,
  );

  let selectedPlugin: PluginFile | null = $derived(
    result && selectedPluginPath
      ? (result.plugins.find((p) => p.path === selectedPluginPath) ?? null)
      : null,
  );

  async function handleFolderPicked(path: string) {
    loading = true;
    error = null;
    selectedIniPath = null;
    selectedPluginPath = null;
    search = "";
    onlyConflicts = false;
    mode = "files";
    try {
      result = await scanFolder(path);
      if (result.files.length > 0) {
        selectedIniPath = result.files[0].path;
      }
      if (result.plugins.length > 0) {
        selectedPluginPath = result.plugins[0].path;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      result = null;
    } finally {
      loading = false;
    }
  }

  function handleConflictFileClick(path: string) {
    selectedIniPath = path;
    mode = "files";
  }

  async function handleExportClick() {
    if (!result) return;
    try {
      const path = await pickSavePath("espector-report.md");
      if (!path) return;
      const content = path.toLowerCase().endsWith(".csv")
        ? buildCsvReport(result, locale.value)
        : buildMarkdownReport(result, locale.value, L);
      await saveReport(path, content);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
  
  async function handleExportPluginsClick() {
    if (!result) return;
    try {
      const path = await pickSavePath("espector-plugins.md");
      if (!path) return;
      const content = path.toLowerCase().endsWith(".csv")
        ? buildCsvReport(result, locale.value)
        : buildMarkdownReport(result, locale.value, L);
      await saveReport(path, content);
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

  let hasAnyContent = $derived(
    result !== null && (result.files.length > 0 || result.plugins.length > 0),
  );
</script>

<div class="h-screen flex flex-col">
  <Toolbar
    rootPath={result?.rootPath ?? ""}
    totalFiles={result?.files.length ?? 0}
    totalPlugins={result?.plugins.length ?? 0}
    totalKeys={result?.totalKeys ?? 0}
    conflictCount={result?.conflictCount ?? 0}
    {loading}
    onFolderPicked={handleFolderPicked}
    onExportClick={handleExportClick}
    onExportPluginsClick={handleExportPluginsClick}
    onMergeClick={handleMergeClick}
    onInfoClick={() => (modal = "about")}
  />

  {#if error}
    <div class="px-4 py-3 bg-[var(--color-danger)]/15 border-b border-[var(--color-danger)]/30 text-sm text-[var(--color-danger)]">
      {L.errors.prefix} {error}
    </div>
  {/if}

  {#if loading}
    <LoadingSkeleton />
  {:else if hasAnyContent && result}
    <TabSwitcher
      {mode}
      filesCount={result.files.length}
      conflictsCount={result.conflicts.length}
      pluginsCount={result.plugins.length}
      onChange={(m) => (mode = m)}
    />

    <main class="flex-1 flex min-h-0">
      {#if mode === "files"}
        <FileList
          files={result.files}
          selectedPath={selectedIniPath}
          {search}
          {onlyConflicts}
          onSearchChange={(v) => (search = v)}
          onOnlyConflictsChange={(v) => (onlyConflicts = v)}
          onSelect={(path) => (selectedIniPath = path)}
        />
        {#if selectedFile}
          <IniViewer file={selectedFile} conflicts={result.conflicts} />
        {:else}
          <EmptyState hasRoot={true} hasSelection={false} />
        {/if}
      {:else if mode === "conflicts"}
        <ConflictList conflicts={result.conflicts} onFileClick={handleConflictFileClick} />
      {:else}
        {#if result.plugins.length > 0}
          <PluginList
            plugins={result.plugins}
            selectedPath={selectedPluginPath}
            onSelect={(path) => (selectedPluginPath = path)}
          />
          {#if selectedPlugin}
            <PluginViewer plugin={selectedPlugin} />
          {:else}
            <EmptyState hasRoot={true} hasSelection={false} />
          {/if}
        {:else}
          <div class="flex-1 flex items-center justify-center bg-[var(--color-bg-base)]">
            <div class="text-center max-w-md px-6">
              <div class="text-5xl mb-4">🧩</div>
              <h2 class="text-lg font-medium text-[var(--color-text-primary)] mb-2">
                {L.plugins.emptyTitle}
              </h2>
              <p class="text-sm text-[var(--color-text-secondary)]">
                {L.plugins.emptyDescription}
              </p>
            </div>
          </div>
        {/if}
      {/if}
    </main>
  {:else}
    <main class="flex-1 flex min-h-0">
      <EmptyState hasRoot={!!result} hasSelection={false} />
    </main>
  {/if}
</div>

{#if modal === "about"}
  <AboutModal
    onClose={() => (modal = "none")}
    onOpenSupport={() => (modal = "support")}
  />
{:else if modal === "support"}
  <SupportModal onClose={() => (modal = "none")} />
{/if}