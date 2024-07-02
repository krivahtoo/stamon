<script>
  import { Toaster as Sonner } from 'svelte-sonner';
  import { mode } from 'mode-watcher';
  import { TriangleAlert, Info, CircleCheckBig, CircleX } from 'lucide-svelte';
</script>

<Sonner
  theme={$mode}
  class="toaster group"
  toastOptions={{
    classes: {
      toast:
        'group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:border-primary/50 group-[.toaster]:shadow-lg',
      description: 'group-[.toast]:text-muted-foreground',
      actionButton: 'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground',
      cancelButton: 'group-[.toast]:bg-muted group-[.toast]:text-muted-foreground',
      closeButton:
        'group-[.toast]:bg-background group-[.toast]:text-muted-foreground group-[.toaster]:border-primary/50'
    }
  }}
  {...$$restProps}
>
  <CircleCheckBig class="pr-1" slot="success-icon" />
  <Info class="pr-1" slot="info-icon" />
  <TriangleAlert class="pr-1" slot="warning-icon" />
  <CircleX class="pr-1" slot="error-icon" />
</Sonner>
