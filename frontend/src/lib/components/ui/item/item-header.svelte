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
	data-slot="item-header"
	class={cn("gap-2 flex basis-full items-center justify-between", className)}
	{...restProps}
>
	{@render children?.()}
</div>