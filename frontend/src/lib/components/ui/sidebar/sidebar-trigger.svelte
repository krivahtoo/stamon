<script>
	import { Button } from "$lib/components/ui/button/index.js";
	import { cn } from "$lib/utils.js";
	import PanelLeft from "@lucide/svelte/icons/panel-left";
	import { useSidebar } from "./context.svelte.js";

	/** @type {any} */
	let {
		ref = $bindable(null),
		class: className,
		onclick,
		...restProps
	} = $props();

	const sidebar = useSidebar();

	/** @param {MouseEvent} e */
	function handleClick(e) {
		onclick?.(e);
		sidebar.toggle();
	}
</script>

<Button
	type="button"
	onclick={handleClick}
	data-sidebar="trigger"
	variant="ghost"
	size="icon"
	class={cn("h-7 w-7", className)}
	{...restProps}
>
	<PanelLeft />
	<span class="sr-only">Toggle Sidebar</span>
</Button>