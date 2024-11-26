<script>
  import ListFilter from 'lucide-svelte/icons/list-filter';
  import File from 'lucide-svelte/icons/file';
  import CirclePlus from 'lucide-svelte/icons/circle-plus';

  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
</script>

<div class="flex items-center">
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
      <DropdownMenu.CheckboxItem>Inactive</DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem>Down</DropdownMenu.CheckboxItem>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
  <div class="ml-auto flex items-center gap-2">
    <Button size="sm" variant="outline" class="h-8 gap-1">
      <File class="h-3.5 w-3.5" />
      <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Export </span>
    </Button>
    <Button size="sm" class="h-8 gap-1" href="/services/new">
      <CirclePlus class="h-3.5 w-3.5" />
      <span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Add Service </span>
    </Button>
  </div>
</div>
