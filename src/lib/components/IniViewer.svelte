<script lang="ts">
  import type { IniFile, ConflictGroup } from "$lib/api";
  import { LL } from "$lib/i18n/state.svelte";
  import Tooltip from "./Tooltip.svelte";

  interface Props {
    file: IniFile;
    conflicts: ConflictGroup[];
  }

  let { file, conflicts }: Props = $props();

  let L = $derived(LL());

  /// Собирает текст тултипа: где ещё встречается ключ, кто победитель.
  function conflictTooltip(sectionName: string, keyName: string): string {
    const full = `${sectionName}.${keyName}`;
    const group = conflicts.find((c) => c.fullKey === full);
    if (!group) return "";

    const others = group.occurrences.filter((o) => o.filePath !== file.path);
    if (others.length === 0) return "";

    const lines: string[] = [];

    // Заголовок: "Также в N файл(ах):"
    lines.push(
      L.viewer.conflictTooltip.replace("{count}", String(others.length)),
    );
    lines.push("");

    // Список других файлов со значениями.
    for (const o of others) {
      lines.push(`${o.fileRelativePath}\n  = ${o.value}`);
    }

    // Инфо о победителе — только если он не текущий файл.
    if (group.winner.filePath !== file.path) {
      lines.push("");
      lines.push(`${L.conflictList.winner}: ${group.winner.fileRelativePath} = ${group.winner.value}`);
    } else {
      lines.push("");
      lines.push(`${L.conflictList.winner}: ${file.relativePath}`);
    }

    return lines.join("\n");
  }

  function isConflict(sectionName: string, keyName: string): boolean {
    return file.conflictingKeys.includes(`${sectionName}.${keyName}`);
  }
</script>

<section class="flex-1 overflow-y-auto bg-[var(--color-bg-base)]">
  <div class="sticky top-0 px-4 py-2 border-b border-[var(--color-border)] bg-[var(--color-bg-surface)] z-10">
    <div class="text-sm font-medium text-[var(--color-text-primary)]">{file.relativePath}</div>
    <div class="text-xs text-[var(--color-text-muted)] truncate" title={file.path}>{file.path}</div>
  </div>

  <div class="p-4 font-mono text-sm leading-relaxed">
    {#each file.sections as section (section.name)}
      <div class="mb-4">
        <div class="text-[var(--color-accent)] font-semibold">
          [{section.name}]
        </div>

        {#each section.comments as comment}
          <div class="text-[var(--color-text-muted)] italic">; {comment}</div>
        {/each}

        {#each section.keys as key (key.name + ":" + key.line)}
          {@const conflict = isConflict(section.name, key.name)}
          {#if conflict}
            <Tooltip text={conflictTooltip(section.name, key.name)}>
              <div class="px-2 py-0.5 rounded -mx-2 bg-[var(--color-danger)]/10 cursor-help">
                <span class="text-[var(--color-danger)]">{key.name}</span>
                <span class="text-[var(--color-text-muted)]"> = </span>
                <span class="text-[var(--color-text-secondary)]">{key.value}</span>
                {#if key.inlineComment}
                  <span class="text-[var(--color-text-muted)] italic"> ; {key.inlineComment}</span>
                {/if}
              </div>
            </Tooltip>
          {:else}
            <div class="px-2 py-0.5 rounded -mx-2">
              <span class="text-[var(--color-text-primary)]">{key.name}</span>
              <span class="text-[var(--color-text-muted)]"> = </span>
              <span class="text-[var(--color-text-secondary)]">{key.value}</span>
              {#if key.inlineComment}
                <span class="text-[var(--color-text-muted)] italic"> ; {key.inlineComment}</span>
              {/if}
            </div>
          {/if}
        {/each}
      </div>
    {/each}
  </div>
</section>