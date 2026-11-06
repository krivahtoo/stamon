<script>
	import { Drawer as DrawerPrimitive } from "vaul-svelte";

	let { ...restProps } = $props();
</script>

<DrawerPrimitive.Portal {...restProps} />