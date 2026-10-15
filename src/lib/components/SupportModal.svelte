<script lang="ts">
  import { LL } from "$lib/i18n/state.svelte";

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  let L = $derived(LL());

  const BTC = "bc1q2ka70s4vtmrskandqj8l4d6n3kdxyxa7kf3wf7";
  const USDT = "TUjY9p6oxKmeCQwNZwMfHqHdXuabaHpgT7";

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }

  async function copy(value: string) {
    try {
      await navigator.clipboard.writeText(value);
    } catch {
      // Игнорируем — если буфер недоступен, ничего страшного.
    }
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
    class="relative w-full max-w-lg rounded-xl
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
      <h1 class="text-lg font-bold text-[var(--color-text-primary)] mb-1">
        {L.support.title}
      </h1>
      <p class="text-xs text-[var(--color-text-muted)] italic mb-6">
        {L.support.subtitle}
      </p>

      <div class="space-y-4">
        <div>
          <div class="text-xs text-[var(--color-text-secondary)] mb-1.5">BTC</div>
          <div class="flex gap-2">
            <input
              type="text"
              readonly
              value={BTC}
              class="flex-1 px-3 py-2 rounded-md text-xs font-mono
                     bg-[var(--color-bg-base)] text-[var(--color-text-primary)]
                     border border-[var(--color-border)] focus:outline-none"
            />
            <button
              onclick={() => copy(BTC)}
              class="px-4 py-2 rounded-md text-xs font-medium
                     bg-[var(--color-accent)] hover:bg-[var(--color-accent-hover)]
                     text-white transition-colors cursor-pointer"
            >
              {L.support.copy}
            </button>
          </div>
        </div>

        <div>
          <div class="text-xs text-[var(--color-text-secondary)] mb-1.5">USDT (TRC-20)</div>
          <div class="flex gap-2">
            <input
              type="text"
              readonly
              value={USDT}
              class="flex-1 px-3 py-2 rounded-md text-xs font-mono
                     bg-[var(--color-bg-base)] text-[var(--color-text-primary)]
                     border border-[var(--color-border)] focus:outline-none"
            />
            <button
              onclick={() => copy(USDT)}
              class="px-4 py-2 rounded-md text-xs font-medium
                     bg-[var(--color-accent)] hover:bg-[var(--color-accent-hover)]
                     text-white transition-colors cursor-pointer"
            >
              {L.support.copy}
            </button>
          </div>
        </div>
      </div>

      <div class="h-px bg-[var(--color-border)] my-5"></div>

      <p class="text-center text-xs text-[var(--color-text-muted)]">
        {L.support.thanks}
      </p>
    </div>
  </div>
</div>