<script>
	import { cn } from "$lib/utils.js";

	/** @type {any} */
	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		...restProps
	} = $props();
</script>

<textarea
	bind:this={ref}
	class={cn(
		"border-input bg-background placeholder:text-muted-foreground flex min-h-[80px] w-full rounded-md border px-3 py-2 text-sm transition-colors duration-160 ease-in-out disabled:cursor-not-allowed disabled:opacity-50",
		className
	)}
	bind:value
	{...restProps}
></textarea>