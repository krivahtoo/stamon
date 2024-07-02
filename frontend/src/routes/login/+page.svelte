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

<Card.Root class="mx-auto my-auto max-w-sm dark:border-primary/50">
  <Card.Header>
    <Card.Title class="text-2xl">Login</Card.Title>
    <Card.Description>Enter your username below to login to your account</Card.Description>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4">
      <Alert.Root variant="destructive">
        <CircleAlert class="h-4 w-4" />
        <Alert.Title>Error</Alert.Title>
        <Alert.Description>Your session has expired. Please login again.</Alert.Description>
      </Alert.Root>
      <div class="grid gap-2">
        <Label for="username">Username</Label>
        <Input id="username" type="text" placeholder="Enter username..." required />
      </div>
      <div class="grid gap-2">
        <Label for="password">Password</Label>
        <Input id="password" {type} placeholder="Enter password..." required />
        <div class="flex items-center space-x-2">
          <Checkbox id="terms" bind:checked={show_pass} aria-labelledby="terms-label" />
          <Label
            id="terms-label"
            for="terms"
            class="cursor-pointer text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Show password
          </Label>
        </div>
      </div>
      <Button type="submit" class="w-full">Login</Button>
    </div>
  </Card.Content>
</Card.Root>
