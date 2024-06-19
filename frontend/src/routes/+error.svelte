<script>
  import { Button } from '$lib/components/ui/button/index.js';
</script>

<!--
  This example requires updating your template:

  ```
  <html class="h-full">
  <body class="h-full">
  ```
-->
<div class="my-auto text-center">
  <p class="text-5xl font-semibold text-primary/80">404</p>
  <h1 class="mb-6 scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
    Page not found
  </h1>
  <p class="text-xl text-muted-foreground">Sorry, we couldn’t find the page you’re looking for.</p>
  <div class="mt-10 flex items-center justify-center gap-x-6">
    <Button>Go back home</Button>
    <Button variant="link">
      Contact support <span aria-hidden="true">&rarr;</span>
    </Button>
  </div>
</div>
