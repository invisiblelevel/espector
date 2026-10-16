<script lang="ts">
  // Скелетон загрузки: имитирует структуру основного интерфейса,
  // пока идёт скан папки. Три колонки: список, содержимое, сводка.
</script>

<div class="flex-1 flex min-h-0 animate-pulse">
  <!-- Левая колонка: список -->
  <aside class="w-80 shrink-0 flex flex-col border-r border-[var(--color-border)] bg-[var(--color-bg-surface)]">
    <div class="px-3 py-2 border-b border-[var(--color-border)]">
      <div class="h-8 rounded-md bg-[var(--color-bg-hover)]"></div>
    </div>
    <div class="px-3 py-2 border-b border-[var(--color-border)]">
      <div class="h-3 w-24 rounded bg-[var(--color-bg-hover)]"></div>
    </div>
    <div class="p-2 space-y-2">
      {#each Array(6) as _}
        <div class="px-3 py-3 space-y-2">
          <div class="h-3 w-3/4 rounded bg-[var(--color-bg-hover)]"></div>
          <div class="h-2 w-1/3 rounded bg-[var(--color-bg-hover)]/60"></div>
        </div>
      {/each}
    </div>
  </aside>

  <!-- Правая колонка: содержимое -->
  <section class="flex-1 bg-[var(--color-bg-base)]">
    <div class="px-4 py-3 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)]">
      <div class="h-3 w-48 rounded bg-[var(--color-bg-hover)] mb-2"></div>
      <div class="h-2 w-64 rounded bg-[var(--color-bg-hover)]/60"></div>
    </div>
    <div class="p-6 space-y-4">
      {#each Array(5) as _}
        <div class="space-y-2">
          <div class="h-3 w-2/3 rounded bg-[var(--color-bg-hover)]"></div>
          <div class="h-3 w-1/2 rounded bg-[var(--color-bg-hover)]/60"></div>
        </div>
      {/each}
    </div>
  </section>
</div>