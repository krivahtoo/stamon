<script>
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";

	let { ...restProps } = $props();
</script>

<AlertDialogPrimitive.Portal {...restProps} />