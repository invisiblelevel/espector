<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    onClose: () => void;
    onOpenSupport: () => void;
  }

  let { onClose, onOpenSupport }: Props = $props();

  let L = $derived(LL());

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-5
         bg-black/65 backdrop-blur-sm"
  onclick={handleBackdrop}
  role="presentation"
>
  <div
    class="relative w-full max-w-md rounded-xl
           bg-[var(--color-bg-surface)] border border-[var(--color-border)]
           shadow-2xl"
    role="dialog"
    aria-modal="true"
    onclick={(e) => e.stopPropagation()}
  >
    <button
      onclick={onClose}
      class="absolute top-3 right-3 w-7 h-7 rounded-md flex items-center justify-center
             text-[var(--color-text-muted)] hover:bg-[var(--color-bg-elevated)] hover:text-[var(--color-text-primary)]
             transition-colors cursor-pointer"
      aria-label={L.about.close}
    >
      ✕
    </button>

    <div class="p-7 pb-6">
      <h1 class="text-xl font-bold text-[var(--color-text-primary)] tracking-tight mb-1">
        ESPector
      </h1>
      <p class="text-xs text-[var(--color-text-muted)] mb-5">
        {L.about.version.replace("{v}", "0.1.0")}
      </p>

      <p class="text-[13.5px] leading-relaxed text-[var(--color-text-secondary)]">
        {L.about.description}
      </p>

      <div class="h-px bg-[var(--color-border)] my-5"></div>

      <button
        onclick={onOpenSupport}
        class="w-full flex items-center justify-center gap-2 px-4 py-3
               bg-[var(--color-bg-base)] border border-[var(--color-border)] rounded-md
               text-[var(--color-text-secondary)] text-sm font-medium
               hover:border-[var(--color-accent)] hover:text-[var(--color-accent)]
               transition-colors cursor-pointer"
      >
        {L.about.support}
      </button>

      <div class="h-px bg-[var(--color-border)] my-5"></div>

      <div class="text-center">
        <p class="text-[11px] text-[var(--color-text-muted)] mb-1">{L.about.license}</p>
        <p class="text-[11px] text-[var(--color-text-muted)]">{L.about.copyright}</p>
      </div>
    </div>
  </div>
</div>