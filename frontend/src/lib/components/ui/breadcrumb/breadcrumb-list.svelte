<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<ol
	bind:this={ref}
	data-slot="breadcrumb-list"
	class={cn("text-muted-foreground gap-1.5 text-xs/relaxed flex flex-wrap items-center wrap-break-word", className)}
	{...restProps}
>
	{@render children?.()}
</ol>