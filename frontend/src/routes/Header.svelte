<script>
  import { CircleUser, Menu, Home, Package2, Search, Sun, Moon } from 'lucide-svelte';

  import { resetMode, setMode } from 'mode-watcher';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import * as Sheet from '$lib/components/ui/sheet/index.js';
</script>

<header
  class="sticky top-0 flex h-16 items-center gap-4 border-b bg-background px-4 dark:border-primary/50 md:px-6"
>
  <nav
    class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6"
  >
    <a href="/" class="flex items-center gap-2 text-lg font-semibold md:text-base">
      <Home class="h-6 w-6" />
      <span class="sr-only">Acme Inc</span>
    </a>
    <a href="/" class="text-muted-foreground transition-colors hover:text-foreground">
      Dashboard
    </a>
    <a href="/" class="text-muted-foreground transition-colors hover:text-foreground"> Orders </a>
    <a href="/" class="text-muted-foreground transition-colors hover:text-foreground"> Products </a>
    <a href="/" class="text-muted-foreground transition-colors hover:text-foreground">
      Customers
    </a>
    <a href="/settings" class="text-foreground transition-colors hover:text-foreground"> Settings </a>
  </nav>
  <Sheet.Root>
    <Sheet.Trigger asChild let:builder>
      <Button variant="outline" size="icon" class="shrink-0 md:hidden" builders={[builder]}>
        <Menu class="h-5 w-5" />
        <span class="sr-only">Toggle navigation menu</span>
      </Button>
    </Sheet.Trigger>
    <Sheet.Content side="left">
      <nav class="grid gap-6 text-lg font-medium">
        <a href="/" class="flex items-center gap-2 text-lg font-semibold">
          <Package2 class="h-6 w-6" />
          <span class="sr-only">Acme Inc</span>
        </a>
        <a href="/" class="text-muted-foreground hover:text-foreground"> Dashboard </a>
        <a href="/" class="text-muted-foreground hover:text-foreground"> Orders </a>
        <a href="/" class="text-muted-foreground hover:text-foreground"> Products </a>
        <a href="/" class="text-muted-foreground hover:text-foreground"> Customers </a>
        <a href="/settings" class="hover:text-foreground"> Settings </a>
      </nav>
    </Sheet.Content>
  </Sheet.Root>
  <div class="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
    <form class="ml-auto flex-1 sm:flex-initial">
      <div class="relative">
        <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input
          type="search"
          placeholder="Search products..."
          class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]"
        />
      </div>
    </form>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild let:builder>
        <Button builders={[builder]} variant="link" size="icon">
          <Sun
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <Moon
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" class="dark:border-primary/50">
        <DropdownMenu.Item on:click={() => setMode('light')}>Light</DropdownMenu.Item>
        <DropdownMenu.Item on:click={() => setMode('dark')}>Dark</DropdownMenu.Item>
        <DropdownMenu.Item on:click={() => resetMode()}>System</DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild let:builder>
        <Button builders={[builder]} variant="secondary" size="icon" class="rounded-full">
          <CircleUser class="h-5 w-5" />
          <span class="sr-only">Toggle user menu</span>
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" class="dark:border-primary/50">
        <DropdownMenu.Label>My Account</DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Item>Settings</DropdownMenu.Item>
        <DropdownMenu.Item>Support</DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item>Logout</DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
</header>