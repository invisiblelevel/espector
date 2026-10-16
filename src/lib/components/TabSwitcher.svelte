<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";
  import { FileText, AlertTriangle, Puzzle } from "lucide-svelte";

  type Mode = "files" | "conflicts" | "plugins";

  interface Props {
    mode: Mode;
    filesCount: number;
    conflictsCount: number;
    pluginsCount: number;
    onChange: (mode: Mode) => void;
  }

  let { mode, filesCount, conflictsCount, pluginsCount, onChange }: Props = $props();

  let L = $derived(LL());
</script>

<div class="flex items-center gap-1 px-4 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)]">
  <button
    onclick={() => onChange("files")}
    class="flex items-center gap-2 px-4 py-2 text-sm border-b-2 transition-all duration-150 -mb-px"
    class:border-[var(--color-accent)]={mode === "files"}
    class:text-[var(--color-text-primary)]={mode === "files"}
    class:border-transparent={mode !== "files"}
    class:text-[var(--color-text-secondary)]={mode !== "files"}
    class:hover:text-[var(--color-text-primary)]={mode !== "files"}
  >
    <FileText size={15} />
    <span>{L.tabs.files}</span>
    <span class="text-[var(--color-text-muted)]">({filesCount})</span>
  </button>

  <button
    onclick={() => onChange("conflicts")}
    class="flex items-center gap-2 px-4 py-2 text-sm border-b-2 transition-all duration-150 -mb-px"
    class:border-[var(--color-accent)]={mode === "conflicts"}
    class:text-[var(--color-text-primary)]={mode === "conflicts"}
    class:border-transparent={mode !== "conflicts"}
    class:text-[var(--color-text-secondary)]={mode !== "conflicts"}
    class:hover:text-[var(--color-text-primary)]={mode !== "conflicts"}
  >
    <AlertTriangle size={15} />
    <span>{L.tabs.conflicts}</span>
    <span class="text-[var(--color-text-muted)]">({conflictsCount})</span>
  </button>

  <button
    onclick={() => onChange("plugins")}
    class="flex items-center gap-2 px-4 py-2 text-sm border-b-2 transition-all duration-150 -mb-px"
    class:border-[var(--color-accent)]={mode === "plugins"}
    class:text-[var(--color-text-primary)]={mode === "plugins"}
    class:border-transparent={mode !== "plugins"}
    class:text-[var(--color-text-secondary)]={mode !== "plugins"}
    class:hover:text-[var(--color-text-primary)]={mode !== "plugins"}
  >
    <Puzzle size={15} />
    <span>{L.tabs.plugins}</span>
    <span class="text-[var(--color-text-muted)]">({pluginsCount})</span>
  </button>
</div>