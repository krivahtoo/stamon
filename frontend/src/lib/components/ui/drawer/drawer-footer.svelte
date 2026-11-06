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
	data-slot="drawer-footer"
	class={cn("gap-2 p-4 mt-auto flex flex-col", className)}
	{...restProps}
>
	{@render children?.()}
</div>