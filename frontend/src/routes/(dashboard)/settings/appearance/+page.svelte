<script>
  import { resetMode, setMode, userPrefersMode } from 'mode-watcher';

  import { Label } from '$lib/components/ui/label/index.js';
  import * as RadioGroup from '$lib/components/ui/radio-group/index.js';

  const options = [
    { value: 'light', label: 'Light' },
    { value: 'dark', label: 'Dark' },
    { value: 'system', label: 'System', hint: 'Follows your device setting.' }
  ];

  /** @param {string | undefined} value */
  function choose(value) {
    if (value === 'light' || value === 'dark') setMode(value);
    else resetMode();
  }
</script>

<svelte:head>
  <title>Appearance settings · Stamon</title>
</svelte:head>

<section class="space-y-4">
  <div>
    <h3 class="text-lg font-semibold">Theme</h3>
    <p class="text-sm text-muted-foreground">Saved in this browser only.</p>
  </div>
  <RadioGroup.Root value={$userPrefersMode} onValueChange={choose} class="grid gap-3">
    {#each options as option (option.value)}
      <div class="flex items-center gap-3">
        <RadioGroup.Item value={option.value} id={`theme-${option.value}`} />
        <Label for={`theme-${option.value}`} class="cursor-pointer">
          {option.label}
          {#if option.hint}
            <span class="ml-1 font-normal text-muted-foreground">{option.hint}</span>
          {/if}
        </Label>
      </div>
    {/each}
  </RadioGroup.Root>
</section>
