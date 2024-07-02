<script>
  import { ModeWatcher } from 'mode-watcher';
  import { Toaster } from "$lib/components/ui/sonner/index.js";

  import '../app.pcss';
</script>

<ModeWatcher />

<Toaster richColors closeButton />

<div class="flex min-h-screen w-full flex-col dark:border-primary/50">
  <slot></slot>
</div>