<script>
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
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
	data-slot="breadcrumb-separator"
	role="presentation"
	aria-hidden="true"
	class={cn("[&>svg]:size-3.5", className)}
	{...restProps}
>
	{#if children}
		{@render children?.()}
	{:else}
		<ChevronRightIcon class="cn-rtl-flip" />
	{/if}
</li>