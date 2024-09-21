<script>
  import { z } from 'zod';

  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import * as HoverCard from '$lib/components/ui/hover-card';
  import CircleHelp from 'lucide-svelte/icons/circle-help';

  let show_pass = false;
  $: type = show_pass ? 'text' : 'password';
</script>

<svelte:head>
  <title>Register</title>
  <meta name="description" content="CVify register page" />
</svelte:head>

<Card.Root class="mx-auto my-auto max-w-sm dark:border-primary/50">
  <Card.Header>
    <Card.Title class="text-xl">Sign Up</Card.Title>
    <Card.Description>Enter your information to create an account</Card.Description>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4">
      <div class="grid gap-2">
        <Label for="username">Username</Label>
        <Input id="username" placeholder="Enter username..." required />
      </div>
      <div class="grid gap-2">
        <div class="flex items-center">
          <Label for="password">Password</Label>
          <HoverCard.Root>
            <HoverCard.Trigger
              class="ml-auto inline-block text-sm underline-offset-1 hover:underline"
            >
              <CircleHelp class="text-secondary-foreground" size={20} />
            </HoverCard.Trigger>
            <HoverCard.Content class="dark:border-primary/40">
              A strong password should contain atleast upper letter, 1 number and 1 symbol
            </HoverCard.Content>
          </HoverCard.Root>
        </div>
        <Input id="password" {type} placeholder="Enter new pass.." />
      </div>
      <div class="grid gap-2">
        <Label for="confirm-password">Confirm Password</Label>
        <Input id="confirm-password" type="password" placeholder="Re-type the pass.." />
      </div>
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
      <Button type="submit" class="w-full">Create an account</Button>
    </div>
  </Card.Content>
</Card.Root>