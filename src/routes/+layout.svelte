<script lang="ts">
  import "../app.css";
  import { detectSystemLocale } from "$lib/i18n/state.svelte";

  let { children } = $props();

  $effect(() => {
    detectSystemLocale();
  });
</script>

{@render children()}