<script>
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { cn } from '$lib/utils.js';
  let { ref = $bindable(null), class: className, children, ...restProps } = $props();

  const hasContent = $derived(!!children);
</script>

<div
  bind:this={ref}
  data-slot="field-separator"
  data-content={hasContent}
  class={cn(
    'relative -my-2 h-5 text-xs/relaxed group-data-[variant=outline]/field-group:-mb-2',
    className
  )}
  {...restProps}
>
  <Separator class="absolute inset-0 top-1/2" />
  {#if children}
    <span
      class="relative mx-auto block w-fit bg-background px-2 text-muted-foreground"
      data-slot="field-separator-content"
    >
      {@render children()}
    </span>
  {/if}
</div>
