<script>
  import X from '@lucide/svelte/icons/x';
  import { cn } from '$lib/utils.js';

  /**
   * Tags as chips. Enter or a comma adds what was typed, and Backspace on an
   * empty input removes the last tag. Limits match the server's.
   * @typedef {Object} Props
   * @property {string[]} [tags]
   * @property {string} [id]
   * @property {number} [max] - The most tags allowed.
   * @property {number} [maxLength] - The longest tag allowed.
   * @property {string} [class]
   */

  /** @type {Props} */
  let { tags = $bindable([]), id, max = 10, maxLength = 32, class: className } = $props();

  let text = $state('');
  let full = $derived(tags.length >= max);

  /** Add the typed tags, skipping empty ones and repeats, ignoring case. */
  function commit() {
    for (const part of text.split(',')) {
      const tag = part.trim();
      if (!tag || tags.length >= max) continue;
      if (tags.some((t) => t.toLowerCase() === tag.toLowerCase())) continue;
      tags = [...tags, tag];
    }
    text = '';
  }

  /** @param {number} index */
  function remove(index) {
    tags = tags.filter((_, i) => i !== index);
  }

  /** @param {KeyboardEvent} event */
  function onkeydown(event) {
    if (event.key === 'Enter' || event.key === ',') {
      event.preventDefault();
      commit();
    } else if (event.key === 'Backspace' && text === '' && tags.length) {
      remove(tags.length - 1);
    }
  }
</script>

<div
  class={cn(
    'flex min-h-10 w-full flex-wrap items-center gap-1.5 rounded-md border border-input bg-background px-2 py-1.5 transition-colors duration-160 focus-within:outline-2 focus-within:outline-offset-3 focus-within:outline-ring',
    className
  )}
>
  {#each tags as tag, index (tag)}
    <span
      class="flex items-center gap-1 rounded-md border bg-surface py-0.5 pr-1 pl-2 text-xs text-foreground"
    >
      {tag}
      <button
        type="button"
        class="rounded-sm text-muted-foreground hover:text-foreground"
        onclick={() => remove(index)}
      >
        <X class="size-3" />
        <span class="sr-only">Remove {tag}</span>
      </button>
    </span>
  {/each}
  <input
    {id}
    bind:value={text}
    {onkeydown}
    onblur={commit}
    maxlength={maxLength}
    disabled={full}
    placeholder={full ? `Up to ${max} tags` : 'Add a tag…'}
    class="h-6 min-w-24 flex-1 bg-transparent px-1 text-sm outline-none placeholder:text-muted-foreground focus-visible:outline-none"
  />
</div>
