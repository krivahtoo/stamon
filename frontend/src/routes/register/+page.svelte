<script>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as HoverCard from '$lib/components/ui/hover-card';
  import { BadgeInfo } from 'lucide-svelte';
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
      <div class="grid grid-cols-2 gap-4">
        <div class="grid gap-2">
          <Label for="first-name">First name</Label>
          <Input id="first-name" placeholder="John" required />
        </div>
        <div class="grid gap-2">
          <Label for="last-name">Last name</Label>
          <Input id="last-name" placeholder="Doe" required />
        </div>
      </div>
      <div class="grid gap-2">
        <Label for="email">Email</Label>
        <Input id="email" type="email" placeholder="m@example.com" required />
      </div>
      <div class="grid gap-2">
        <div class="flex items-center">
          <Label for="password">Password</Label>
          <HoverCard.Root>
            <HoverCard.Trigger
              class="ml-auto inline-block text-sm underline-offset-1 hover:underline"
            >
              <BadgeInfo size={20} />
            </HoverCard.Trigger>
            <HoverCard.Content class="dark:border-primary/40">
              A strong password should contain atleast upper letter, 1 number and 1 symbol
            </HoverCard.Content>
          </HoverCard.Root>
        </div>
        <Input id="password" type="password" placeholder="Enter new pass.." />
      </div>
      <!-- <div class="grid gap-2"> -->
      <!--   <Label for="confirm-password">Confirm Password</Label> -->
      <!--   <Input id="confirm-password" type="password" placeholder="Re-type the pass.." /> -->
      <!-- </div> -->
      <Button type="submit" class="w-full">Create an account</Button>
      <Button variant="outline" class="w-full">Sign up with GitHub</Button>
    </div>
    <div class="mt-4 text-center text-sm">
      Already have an account?
      <a href="/login" class="underline"> Sign in </a>
    </div>
  </Card.Content>
</Card.Root>
