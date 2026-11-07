<script>
	import MoreHorizontalIcon from '@lucide/svelte/icons/more-horizontal';
	import { cn } from "$lib/utils.js";
	let {
		ref = $bindable(null),
		class: className,
		...restProps
	} = $props();
</script>

<span
	bind:this={ref}
	data-slot="breadcrumb-ellipsis"
	role="presentation"
	aria-hidden="true"
	class={cn("size-4 [&>svg]:size-3.5 flex items-center justify-center", className)}
	{...restProps}
>
	<MoreHorizontalIcon  />
	<span class="sr-only">More</span>
</span>