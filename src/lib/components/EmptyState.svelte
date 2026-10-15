<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    hasRoot: boolean;
    hasSelection: boolean;
  }

  let { hasRoot, hasSelection }: Props = $props();

  let L = $derived(LL());
</script>

<div class="flex-1 flex items-center justify-center bg-[var(--color-bg-base)]">
  <div class="text-center max-w-md px-6">
    {#if !hasRoot}
      <div class="text-5xl mb-4">🔍</div>
      <h2 class="text-lg font-medium text-[var(--color-text-primary)] mb-2">
        {L.emptyState.pickFolderTitle}
      </h2>
      <p class="text-sm text-[var(--color-text-secondary)]">
        {L.emptyState.pickFolderDescription}
      </p>
    {:else if !hasSelection}
      <div class="text-5xl mb-4">📄</div>
      <h2 class="text-lg font-medium text-[var(--color-text-primary)] mb-2">
        {L.emptyState.pickFileTitle}
      </h2>
      <p class="text-sm text-[var(--color-text-secondary)]">
        {L.emptyState.pickFileDescription}
      </p>
    {/if}
  </div>
</div>