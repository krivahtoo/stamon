<script>
  import { cn } from '$lib/utils.js';
  let className = undefined;
  export { className as class };
</script>

<div
  class={cn(
    'rounded-lg border bg-card text-card-foreground shadow-sm dark:border-primary/50',
    className
  )}
  {...$$restProps}
>
  <slot />
</div>
