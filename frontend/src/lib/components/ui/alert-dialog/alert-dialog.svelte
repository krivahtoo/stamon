<script>
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";

	let { open = $bindable(false), ...restProps } = $props();
</script>

<AlertDialogPrimitive.Root bind:open {...restProps} />