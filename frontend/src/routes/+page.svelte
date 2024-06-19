<script>
  import Counter from './Counter.svelte';
  import welcome from '$lib/images/svelte-welcome.webp';
  import { Button } from '$lib/components/ui/button/index.js';
  import welcome_fallback from '$lib/images/svelte-welcome.png';
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="flex flex-1 items-center justify-center rounded-lg border border-dashed shadow-sm">
	<div class="flex flex-col items-center gap-1 text-center">
		<h3 class="text-2xl font-bold tracking-tight">You have no products</h3>
		<p class="text-sm text-muted-foreground">
			You can start selling as soon as you add a product.
		</p>
		<Button class="mt-4">Add Product</Button>
	</div>
</div>


<section>
  <h1>
    <span class="welcome">
      <picture>
        <source srcset={welcome} type="image/webp" />
        <img src={welcome_fallback} alt="Welcome" />
      </picture>
    </span>

    to your new<br />SvelteKit app
  </h1>

  <h2>
    try editing <strong>src/routes/+page.svelte</strong>
  </h2>

  <Counter />
</section>