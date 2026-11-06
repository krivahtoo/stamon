<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		children,
		child,
		class: className,
		...restProps
	} = $props();

	const mergedProps = $derived({
		class: cn(
			"text-sidebar-foreground/70 ring-sidebar-ring h-8 rounded-md px-2 text-xs transition-[margin,opacity] duration-200 ease-linear group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0 focus-visible:ring-2 [&>svg]:size-4 flex shrink-0 items-center outline-hidden [&>svg]:shrink-0",
			className
		),
		"data-slot": "sidebar-group-label",
		"data-sidebar": "group-label",
		...restProps,
	});
</script>

{#if child}
	{@render child({ props: mergedProps })}
{:else}
	<div bind:this={ref} {...mergedProps}>
		{@render children?.()}
	</div>
{/if}