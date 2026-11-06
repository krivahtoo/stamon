<script>
	import { Dialog as SheetPrimitive } from "bits-ui";

	/** @type {any} */
	let { ...restProps } = $props();
</script>

<SheetPrimitive.Portal {...restProps} />
