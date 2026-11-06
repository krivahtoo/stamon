<script>
	import * as FormPrimitive from "formsnap";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	} = $props();
</script>

<FormPrimitive.Legend
	bind:ref
	class={cn("text-sm leading-none font-medium data-[fs-error]:text-destructive", className)}
	{...restProps}
/>