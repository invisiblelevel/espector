<script lang="ts">
  interface Props {
    text: string;
    children: import("svelte").Snippet;
  }

  let { text, children }: Props = $props();

  let visible = $state(false);
  let x = $state(0);
  let y = $state(0);

  function handleMouseMove(e: MouseEvent) {
    x = e.clientX;
    y = e.clientY;
  }
</script>

<span
  class="contents"
  onmouseenter={() => (visible = true)}
  onmouseleave={() => (visible = false)}
  onmousemove={handleMouseMove}
>
  {@render children()}
</span>

{#if visible}
  <div
    class="fixed z-50 pointer-events-none px-2.5 py-1.5 rounded-md text-xs
           bg-[var(--color-bg-elevated)] text-[var(--color-text-primary)]
           border border-[var(--color-border)] shadow-lg whitespace-pre-line max-w-xs"
    style="left: {x + 12}px; top: {y + 12}px;"
  >
    {text}
  </div>
{/if}