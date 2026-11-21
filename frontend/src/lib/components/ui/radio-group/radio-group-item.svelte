<script>
	import { RadioGroup as RadioGroupPrimitive } from "bits-ui";
	import Circle from "@lucide/svelte/icons/circle";
	import { cn } from "$lib/utils.js";

	/** @type {any} */
	let {
		ref = $bindable(null),
		class: className,
		...restProps
	} = $props();
</script>

<RadioGroupPrimitive.Item
	bind:ref
	class={cn(
		"border-primary text-primary aspect-square size-4 rounded-full border disabled:cursor-not-allowed disabled:opacity-50",
		className
	)}
	{...restProps}
>
	{#snippet children({ checked })}
		<div class="flex items-center justify-center">
			{#if checked}
				<Circle class="size-2.5 fill-current text-current" />
			{/if}
		</div>
	{/snippet}
</RadioGroupPrimitive.Item>