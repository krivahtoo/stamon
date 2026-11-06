<script>
	import { cn } from "$lib/utils.js";

	/** @type {any} */
	let {
		ref = $bindable(null),
		class: className,
		level = 5,
		children,
		...restProps
	} = $props();
</script>

<div
	role="heading"
	aria-level={level}
	bind:this={ref}
	class={cn("mb-1 font-medium leading-none tracking-tight", className)}
	{...restProps}
>
	{@render children?.()}
</div>