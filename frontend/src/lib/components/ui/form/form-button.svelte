<script>
	import * as Button from "$lib/components/ui/button/index.js";

	/** @type {any} */
	let { ref = $bindable(null), ...restProps } = $props();
</script>

<Button.Root bind:ref type="submit" {...restProps} />