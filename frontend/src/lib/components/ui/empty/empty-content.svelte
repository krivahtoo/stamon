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
	data-slot="empty-content"
	class={cn(
		"gap-2 text-xs/relaxed flex w-full max-w-sm min-w-0 flex-col items-center text-balance",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>