<script>
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	} = $props();
</script>

<thead
	bind:this={ref}
	data-slot="table-header"
	class={cn("bg-surface [&_tr]:border-b [&_tr]:hover:bg-transparent", className)}
	{...restProps}
>
	{@render children?.()}
</thead>