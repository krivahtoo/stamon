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
	data-slot="dialog-header"
	class={cn("gap-0.5 flex flex-col pr-8", className)}
	{...restProps}
>
	{@render children?.()}
</div>