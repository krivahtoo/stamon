<script module>
	import { tv } from "tailwind-variants";

	export const itemVariants = tv({
		base: "[a]:hover:bg-muted rounded-md border text-xs/relaxed group/item flex w-full flex-wrap items-center transition-colors duration-100 [a]:transition-colors",
		variants: {
			variant: {
				default: "border-transparent",
				outline: "border-border",
				muted: "bg-muted/50 border-transparent",
			},
			size: {
				default: "gap-2.5 px-3 py-2.5",
				sm: "gap-2.5 px-3 py-2.5",
				xs: "gap-2.5 px-2.5 py-2 in-data-[slot=dropdown-menu-content]:p-0",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

</script>

<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		child,
		variant,
		size,
		...restProps
	} = $props();

	const mergedProps = $derived({
		class: cn(itemVariants({ variant, size }), className),
		"data-slot": "item",
		"data-variant": variant,
		"data-size": size,
		...restProps,
	});
</script>

{#if child}
	{@render child({ props: mergedProps })}
{:else}
	<div bind:this={ref} {...mergedProps}>
		{@render mergedProps.children?.()}
	</div>
{/if}