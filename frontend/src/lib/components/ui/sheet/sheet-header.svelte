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
	data-slot="sheet-header"
	class={cn("gap-1.5 p-6 flex flex-col", className)}
	{...restProps}
>
	{@render children?.()}
</div>