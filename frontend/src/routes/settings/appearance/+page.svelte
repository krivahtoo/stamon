<script>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import * as Alert from '$lib/components/ui/alert/index.js';

  let show_pass = false;
  $: type = show_pass ? 'text' : 'password';
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="CVify login page" />
</svelte:head>

<div class="grid gap-6">
  <Card.Root
    data-x-chunk-name="dashboard-04-chunk-2"
    data-x-chunk-description="A form to update the plugins directory with a checkbox to allow administrators to change the directory."
  >
    <Card.Header>
      <Card.Title>Plugins Directory</Card.Title>
      <Card.Description>
        The directory within your project, in which your plugins are located.
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <form class="flex flex-col gap-4">
        <Input placeholder="Project Name" value="/content/plugins" />
        <div class="flex items-center space-x-2">
          <Checkbox id="include" checked={true} />
          <label
            for="include"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Allow administrators to change the directory.
          </label>
        </div>
      </form>
    </Card.Content>
    <Card.Footer class="border-t px-6 py-4">
      <Button>Save</Button>
    </Card.Footer>
  </Card.Root>
</div>