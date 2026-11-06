<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<th bind:this={ref} data-slot="table-head" class={cn("text-muted-foreground h-10 px-4 text-left align-middle text-xs font-semibold tracking-[0.08em] uppercase whitespace-nowrap first:pl-5 last:pr-5 [&:has([role=checkbox])]:pr-0", className)} {...restProps}>
	{@render children?.()}
</th>