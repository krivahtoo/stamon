<script>
  import { Input } from '$lib/components/ui/input/index.js';
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), value = $bindable(''), class: className, ...restProps } = $props();
</script>

<Input
  bind:ref
  bind:value
  data-slot="sidebar-input"
  data-sidebar="input"
  class={cn('h-8 w-full border-input bg-muted/20 dark:bg-muted/30', className)}
  {...restProps}
/>
