<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    mode: "files" | "conflicts";
    filesCount: number;
    conflictsCount: number;
    onChange: (mode: "files" | "conflicts") => void;
  }

  let { mode, filesCount, conflictsCount, onChange }: Props = $props();

  let L = $derived(LL());
</script>

<div class="flex items-center gap-1 px-4 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)]">
  <button
    onclick={() => onChange("files")}
    class="px-4 py-2 text-sm border-b-2 transition-colors -mb-px"
    class:border-[var(--color-accent)]={mode === "files"}
    class:text-[var(--color-text-primary)]={mode === "files"}
    class:border-transparent={mode !== "files"}
    class:text-[var(--color-text-secondary)]={mode !== "files"}
    class:hover:text-[var(--color-text-primary)]={mode !== "files"}
  >
    {L.tabs.files} <span class="text-[var(--color-text-muted)]">({filesCount})</span>
  </button>

  <button
    onclick={() => onChange("conflicts")}
    class="px-4 py-2 text-sm border-b-2 transition-colors -mb-px"
    class:border-[var(--color-accent)]={mode === "conflicts"}
    class:text-[var(--color-text-primary)]={mode === "conflicts"}
    class:border-transparent={mode !== "conflicts"}
    class:text-[var(--color-text-secondary)]={mode !== "conflicts"}
    class:hover:text-[var(--color-text-primary)]={mode !== "conflicts"}
  >
    {L.tabs.conflicts} <span class="text-[var(--color-text-muted)]">({conflictsCount})</span>
  </button>
</div>