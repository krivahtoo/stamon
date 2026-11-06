<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<tr
	bind:this={ref}
	data-slot="table-row"
	class={cn("hover:bg-surface data-[state=selected]:bg-surface border-b transition-colors duration-160 ease-in-out has-aria-expanded:bg-surface", className)}
	{...restProps}
>
	{@render children?.()}
</tr>