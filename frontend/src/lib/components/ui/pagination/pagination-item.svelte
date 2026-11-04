<script>
	let {
		ref = $bindable(null),
		children,
		...restProps
	} = $props();
</script>

<li bind:this={ref} {...restProps}>
	{@render children?.()}
</li>