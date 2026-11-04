<script>
	import { Select as SelectPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	} = $props();
</script>

<SelectPrimitive.GroupHeading
	bind:ref
	class={cn("py-1.5 pl-8 pr-2 text-sm font-semibold", className)}
	{...restProps}
/>