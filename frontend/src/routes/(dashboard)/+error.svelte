<script>
  import { page } from '$app/stores';

  import { Button } from '$lib/components/ui/button/index.js';
</script>

<div class="my-auto text-center">
  <p class="text-5xl font-semibold text-primary/80">{$page.status}</p>
  <h1 class="mb-6 scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
    {$page.error?.message}
  </h1>
  {#if $page.status === 404}
    <p class="text-xl text-muted-foreground">
      Sorry, we couldn’t find what you’re looking for. Try refreshing the page.
    </p>
  {/if}
  <div class="mt-10 flex items-center justify-center gap-x-6">
    <Button on:click={() => history.back()}>Go back</Button>
  </div>
</div>
