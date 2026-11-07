<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<li
	bind:this={ref}
	data-slot="breadcrumb-item"
	class={cn("gap-1 inline-flex items-center", className)}
	{...restProps}
>
	{@render children?.()}
</li>