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
	data-slot="card-title"
	class={cn("text-base font-bold tracking-[-0.015em]", className)}
	{...restProps}
>
	{@render children?.()}
</div>