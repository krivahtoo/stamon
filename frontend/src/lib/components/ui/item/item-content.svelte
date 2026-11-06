<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<div
	bind:this={ref}
	data-slot="item-content"
	class={cn(
		"gap-1 group-data-[size=xs]/item:gap-0.5 flex flex-1 flex-col [&+[data-slot=item-content]]:flex-none",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>