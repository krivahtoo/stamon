<script>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import LoaderCircle from 'lucide-svelte/icons/loader-circle';
  import * as Alert from '$lib/components/ui/alert/index.js';
  import user from '$lib/store/user.js';
  import { token } from '$lib/store/auth.js';
  import { goto } from '$app/navigation';
  import { cfetch } from '$lib/utils.js';
  import { onMount } from 'svelte';
  import { z } from 'zod';

  /** @type {string[]} */
  let errors = [];

  let is_loading = false;

  let show_pass = false;
  $: type = show_pass ? 'text' : 'password';

  let newUser = {
    username: '',
    password: ''
  };

  /** @type {{ username: string[], password: string[] }} */
  let fieldErrors = {
    username: [],
    password: []
  };

  const baseSchema = z.object({
    username: z.string().min(4),
    password: z.string().min(6)
  });

  /** @type {import('./$types').Snapshot<string>} */
  export const snapshot = {
    capture: () => newUser.username,
    restore: (value) => (newUser.username = value)
  };

  async function login() {
    is_loading = true;
    errors = [];
    let valid_res = baseSchema.safeParse(newUser);
    if (!valid_res.success) {
      // handle error then return
      // console.log(valid_res.error.flatten());
      fieldErrors = { ...{ username: [], password: [] }, ...valid_res.error.flatten().fieldErrors };
    } else {
      try {
        let res = await cfetch('/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(newUser),
          credentials: 'same-origin'
        });
        // console.log(res);
        if (res.ok) {
          let d = await res.json();
          token.set(d.token);
          let ures = await cfetch('/user', { credentials: 'same-origin' });
          if (ures.ok) {
            let data = await ures.json();
            user.set(data.user);
            goto('/');
          } else {
            let data = await ures.json();
            errors = [data.error];
          }
        } else {
          let data = await res.json();
          errors = [data.error];
        }
      } catch (e) {
        // console.log(e);
        errors = [`${e}`];
      } finally {
        newUser.password = '';
        is_loading = false;
      }
    }
  }

  onMount(async () => {
    try {
      let res = await cfetch('/user', { credentials: 'same-origin' });
      if (res.ok) {
        let data = await res.json();
        user.set(data.user);
        goto('/');
      } else {
        user.set(null);
        token.set(null);
      }
    } catch {
      user.set(null);
      token.set(null);
    }
  });
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="stamon login page" />
</svelte:head>

<Card.Root class="mx-auto my-auto max-w-sm dark:border-primary/50">
  <Card.Header>
    <Card.Title class="text-2xl">Login</Card.Title>
    <Card.Description>Enter your username below to login to your account</Card.Description>
  </Card.Header>
  <Card.Content>
    <form on:submit|preventDefault={login}>
      <div class="grid gap-4">
        {#if errors.length > 0}
          <Alert.Root variant="destructive">
            <CircleAlert class="h-4 w-4" />
            <Alert.Title>Error</Alert.Title>
            {#each errors as err, idx (idx)}
              <Alert.Description>{err}</Alert.Description>
            {/each}
          </Alert.Root>
        {/if}
        <div class="grid gap-2">
          <Label for="username">Username</Label>
          <Input
            id="username"
            bind:value={newUser.username}
            type="text"
            placeholder="Enter username..."
            required
          />
          {#each fieldErrors.username as er}
            <p class="text-xs text-destructive">{er}</p>
          {/each}
        </div>
        <div class="grid gap-2">
          <Label for="password">Password</Label>
          <Input
            id="password"
            bind:value={newUser.password}
            {type}
            placeholder="Enter password..."
            required
          />
          {#each fieldErrors.password as er}
            <p class="text-xs text-destructive">{er}</p>
          {/each}
          <div class="flex items-center space-x-2">
            <Checkbox id="show-pass" bind:checked={show_pass} aria-labelledby="show-pass-label" />
            <Label
              id="show-pass-label"
              for="show-pass"
              class="cursor-pointer text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
              Show password
            </Label>
          </div>
        </div>
        <Button type="submit" disabled={!newUser.username || !newUser.password || is_loading} class="w-full">
          {#if is_loading}
            <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
          {/if}
          Login
        </Button>
      </div>
    </form>
  </Card.Content>
</Card.Root>
