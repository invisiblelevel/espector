<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    value: string;
    onlyConflicts: boolean;
    onValueChange: (v: string) => void;
    onOnlyConflictsChange: (v: boolean) => void;
  }

  let { value, onlyConflicts, onValueChange, onOnlyConflictsChange }: Props = $props();

  let L = $derived(LL());
</script>

<div class="px-3 py-2 border-b border-[var(--color-border)] space-y-2">
  <input
    type="text"
    placeholder={L.fileList.searchPlaceholder}
    value={value}
    oninput={(e) => onValueChange((e.currentTarget as HTMLInputElement).value)}
    class="w-full px-2.5 py-1.5 rounded-md text-sm
           bg-[var(--color-bg-base)] text-[var(--color-text-primary)]
           border border-[var(--color-border)]
           focus:outline-none focus:border-[var(--color-accent)]
           placeholder:text-[var(--color-text-muted)]"
  />

  <label class="flex items-center gap-2 text-xs text-[var(--color-text-secondary)] cursor-pointer select-none">
    <input
      type="checkbox"
      checked={onlyConflicts}
      onchange={(e) => onOnlyConflictsChange((e.currentTarget as HTMLInputElement).checked)}
      class="accent-[var(--color-accent)]"
    />
    {L.fileList.onlyConflicts}
  </label>
</div>