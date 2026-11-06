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
	data-slot="item-actions"
	class={cn("gap-2 flex items-center", className)}
	{...restProps}
>
	{@render children?.()}
</div>