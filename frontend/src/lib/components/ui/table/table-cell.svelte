<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<td bind:this={ref} data-slot="table-cell" class={cn("h-12 px-4 py-2 align-middle whitespace-nowrap first:pl-5 last:pr-5 [&:has([role=checkbox])]:pr-0", className)} {...restProps}>
	{@render children?.()}
</td>