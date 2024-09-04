<script>
  import {
    CircleUser,
    Menu,
    Home,
    Package2,
    Search,
    Sun,
    Moon,
    ShoppingCart,
    LineChart,
    Users,
    Package,
    Languages,
    Settings,
    LogOut,
    UserRoundCog
  } from 'lucide-svelte';

  import { resetMode, setMode } from 'mode-watcher';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import * as Sheet from '$lib/components/ui/sheet/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Avatar from '$lib/components/ui/avatar/index.js';
</script>

<header class="flex h-14 items-center gap-4 border-b dark:border-primary/50 bg-muted/40 px-4 lg:h-[60px] lg:px-6">
  <Sheet.Root>
    <Sheet.Trigger asChild let:builder>
      <Button variant="outline" size="icon" class="shrink-0 md:hidden mr-auto" builders={[builder]}>
        <Menu class="h-5 w-5" />
        <span class="sr-only">Toggle navigation menu</span>
      </Button>
    </Sheet.Trigger>
    <Sheet.Content side="left" class="flex flex-col">
      <nav class="grid gap-2 text-lg font-medium">
        <a href="/" class="flex items-center gap-2 text-lg font-semibold">
          <Package2 class="h-6 w-6" />
          <span class="sr-only">Stamon</span>
        </a>
        <a
          href="/"
          class="mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 text-muted-foreground hover:text-foreground"
        >
          <Home class="h-5 w-5" />
          Dashboard
        </a>
        <a
          href="/"
          class="mx-[-0.65rem] flex items-center gap-4 rounded-xl bg-muted px-3 py-2 text-foreground hover:text-foreground"
        >
          <ShoppingCart class="h-5 w-5" />
          Orders
          <Badge class="ml-auto flex h-6 w-6 shrink-0 items-center justify-center rounded-full">
            6
          </Badge>
        </a>
        <a
          href="/"
          class="mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 text-muted-foreground hover:text-foreground"
        >
          <Package class="h-5 w-5" />
          Products
        </a>
        <a
          href="/"
          class="mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 text-muted-foreground hover:text-foreground"
        >
          <Users class="h-5 w-5" />
          Customers
        </a>
        <a
          href="/"
          class="mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 text-muted-foreground hover:text-foreground"
        >
          <LineChart class="h-5 w-5" />
          Analytics
        </a>
      </nav>
      <div class="mt-auto">
        <Card.Root>
          <Card.Header>
            <Card.Title>Upgrade to Pro</Card.Title>
            <Card.Description>
              Unlock all features and get unlimited access to our support team.
            </Card.Description>
          </Card.Header>
          <Card.Content>
            <Button size="sm" class="w-full">Upgrade</Button>
          </Card.Content>
        </Card.Root>
      </div>
    </Sheet.Content>
  </Sheet.Root>
  <div class="hidden sm:block w-full flex-1">
    <form>
      <div class="relative">
        <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input
          type="search"
          placeholder="Search monitors..."
          class="w-full appearance-none bg-background pl-8 shadow-none md:w-2/3 lg:w-1/3"
        />
      </div>
    </form>
  </div>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="link" size="icon" class="hidden sm:flex">
        <Languages class="h-5 w-5" />
        <span class="sr-only">Toggle user menu</span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>Select language</DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>English</DropdownMenu.Item>
      <DropdownMenu.Item>French</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
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
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>
        <div class="flex flex-col space-y-1">
          <p class="text-sm font-medium leading-none">krivah</p>
          <p class="text-xs leading-none text-muted-foreground">Admin</p>
        </div>
      </DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>
        <UserRoundCog class="text-foreground-alt mr-2 size-5" />
        Account
      </DropdownMenu.Item>
      <DropdownMenu.Item href="/settings">
        <Settings class="mr-2 size-5 text-foreground" />
        Settings
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>
        <LogOut class="text-foreground-alt mr-2 size-5" />
        Logout
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</header>