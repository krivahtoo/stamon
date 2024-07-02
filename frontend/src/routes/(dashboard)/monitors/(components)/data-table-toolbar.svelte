<script>
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { CirclePlus, File, ListFilter } from 'lucide-svelte';
</script>

<div class="flex items-center">
  <!-- <Tabs.List> -->
  <!--   <Tabs.Trigger value="all">All</Tabs.Trigger> -->
  <!--   <Tabs.Trigger value="active">Active</Tabs.Trigger> -->
  <!--   <Tabs.Trigger value="draft">Draft</Tabs.Trigger> -->
  <!--   <Tabs.Trigger value="archived" class="hidden sm:flex">Archived</Tabs.Trigger> -->
  <!-- </Tabs.List> -->
  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" size="sm" class="h-8 gap-1">
        <ListFilter class="h-3.5 w-3.5" />
        <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Filter </span>
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>Filter by</DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.CheckboxItem checked>Active</DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem>Draft</DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem>Archived</DropdownMenu.CheckboxItem>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <div class="ml-auto flex items-center gap-2">
    <Button size="sm" variant="outline" class="h-8 gap-1">
      <File class="h-3.5 w-3.5" />
      <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Export </span>
    </Button>
    <Button size="sm" class="h-8 gap-1">
      <CirclePlus class="h-3.5 w-3.5" />
      <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Add Monitor </span>
    </Button>
  </div>
</div>
