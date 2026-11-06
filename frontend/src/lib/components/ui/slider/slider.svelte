<script>
	import { Slider as SliderPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	/** @type {any} */
	let {
		ref = $bindable(null),
		value = $bindable(),
		orientation = "horizontal",
		class: className,
		...restProps
	} = $props();
</script>

<!--
Discriminated Unions + Destructing (required for bindable) do not
get along, so we shut typescript up by casting `value` to `never`.
-->
<SliderPrimitive.Root
	bind:ref
	bind:value={value}
	{orientation}
	class={cn(
		"relative flex touch-none select-none items-center data-[orientation='vertical']:h-full data-[orientation='vertical']:min-h-44 data-[orientation='horizontal']:w-full data-[orientation='vertical']:w-auto data-[orientation='vertical']:flex-col",
		className
	)}
	{...restProps}
>
	{#snippet children({ thumbs })}
		<span
			data-orientation={orientation}
			class="bg-secondary relative grow overflow-hidden rounded-full data-[orientation='horizontal']:h-2 data-[orientation='vertical']:h-full data-[orientation='horizontal']:w-full data-[orientation='vertical']:w-2"
		>
			<SliderPrimitive.Range
				class="bg-primary absolute data-[orientation='horizontal']:h-full data-[orientation='vertical']:w-full"
			/>
		</span>
		{#each thumbs as thumb (thumb)}
			<SliderPrimitive.Thumb
				index={thumb}
				class="border-primary bg-background block size-5 rounded-full border-2 transition-colors disabled:pointer-events-none disabled:opacity-50"
			/>
		{/each}
	{/snippet}
</SliderPrimitive.Root>