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
	data-slot="item-title"
	class={cn("gap-2 text-xs/relaxed leading-snug font-medium underline-offset-4 line-clamp-1 flex w-fit items-center", className)}
	{...restProps}
>
	{@render children?.()}
</div>