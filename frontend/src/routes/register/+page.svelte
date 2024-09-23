<script>
  import { z } from 'zod';

  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { Checkbox } from '$lib/components/ui/checkbox/index.js';
  import * as Alert from '$lib/components/ui/alert/index.js';
  import * as HoverCard from '$lib/components/ui/hover-card';
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import CircleHelp from 'lucide-svelte/icons/circle-help';
  import LoaderCircle from 'lucide-svelte/icons/loader-circle';
  import { cfetch } from '@/utils';
  import { goto } from '$app/navigation';
  import { toast } from 'svelte-sonner';

  /** @type {string[]} */
  let errors = [];

  let is_loading = false;
  let show_pass = false;
  $: type = show_pass ? 'text' : 'password';

  let confirm = '';
  let newUser = {
    username: '',
    password: '',
    confirm: ''
  };

  /** @type {{ username: string[], password: string[], confirm: string[] }} */
  let fieldErrors = {
    username: [],
    password: [],
    confirm: []
  };

  const baseSchema = z
    .object({
      username: z.string().min(4, { message: 'Username must contain at least 4 character(s)' }),
      password: z
        .string()
        .min(6, { message: 'Password must contain at least 6 character(s)' })
        .regex(/[A-Z]/, { message: 'Password must contain at least one uppercase letter' })
        .regex(/[0-9]/, { message: 'Password must contain at least one number' })
        .regex(/[^A-Za-z0-9]/, { message: 'Password must contain at least one symbol' }),
      confirm: z.string()
    })
    .refine((o) => o.password === o.confirm, {
      message: 'Passwords do not match',
      path: ['confirm']
    });

  async function register() {
    is_loading = true;
    let valid_res = baseSchema.safeParse(newUser);
    // console.log(valid_res);
    if (!valid_res.success) {
      console.log(valid_res.error.flatten());
      // @ts-ignore
      fieldErrors = {
        ...{ username: [], password: [], confirm: [] },
        ...valid_res.error.flatten().fieldErrors
      };
      toast.error('Validation Errors', {
        description: 'Please correct the errors in the form and try again'
      });
      is_loading = false;
    } else {
      try {
        let res = await cfetch('/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(newUser),
          credentials: 'same-origin'
        });
        console.log(res);
        if (res.ok) {
          goto('/login');
        } else {
          let data = await res.json();
          errors = [data.error];
        }
      } catch (e) {
        // console.log(e);
        toast.error('Network error', {
          description:
            'Failed to connect to server. Please check your internet connection and try again.'
        });
        errors = [`${e}`];
      } finally {
        newUser.password = '';
        is_loading = false;
      }
    }
  }
</script>

<svelte:head>
  <title>Register</title>
  <meta name="description" content="stamon register page" />
</svelte:head>

<Card.Root class="mx-auto my-auto dark:border-primary/50">
  <Card.Header>
    <Card.Title class="text-xl">Sign Up</Card.Title>
    <Card.Description>Enter your information to create an account</Card.Description>
  </Card.Header>
  <Card.Content>
    <form on:submit|preventDefault={register}>
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
        <div class="grid gap-1">
          <Label for="username">Username</Label>
          <Input
            id="username"
            class={fieldErrors.username.length > 0 ? 'border-red-300' : ''}
            bind:value={newUser.username}
            placeholder="Enter username..."
            required
          />
          {#each fieldErrors.username as er}
            <p class="text-xs text-destructive">{er}</p>
          {/each}
        </div>
        <div class="grid gap-1">
          <div class="flex items-center">
            <Label for="password">Password</Label>
            <HoverCard.Root>
              <HoverCard.Trigger
                class="ml-auto inline-block text-sm underline-offset-1 hover:underline"
              >
                <CircleHelp class="text-secondary-foreground dark:text-secondary" size={20} />
              </HoverCard.Trigger>
              <HoverCard.Content class="dark:border-primary/40">
                A strong password must contain at least one uppercase letter, 1 number, and 1 symbol
              </HoverCard.Content>
            </HoverCard.Root>
          </div>
          <Input
            id="password"
            class={fieldErrors.password.length > 0 ? 'border-red-300' : ''}
            {type}
            bind:value={newUser.password}
            placeholder="Enter new pass.."
          />
          {#each fieldErrors.password as er}
            <p class="text-xs text-destructive">{er}</p>
          {/each}
        </div>
        <div class="grid gap-1">
          <Label for="confirm-password">Confirm Password</Label>
          <Input
            id="confirm-password"
            class={fieldErrors.confirm.length > 0 ? 'border-red-300' : ''}
            type="password"
            bind:value={confirm}
            placeholder="Re-type the pass.."
          />
          {#each fieldErrors.confirm as er}
            <p class="text-xs text-destructive">{er}</p>
          {/each}
        </div>
        <div class="flex items-center space-x-2">
          <Checkbox
            id="show-password"
            bind:checked={show_pass}
            aria-labelledby="show-password-label"
          />
          <Label
            id="show-password-label"
            for="show-password"
            class="cursor-pointer text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Show password
          </Label>
        </div>
        <Button
          type="submit"
          disabled={!newUser.username || !newUser.password || !confirm || is_loading}
          class="w-full"
        >
          {#if is_loading}
            <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
          {:else}
            Create an account
          {/if}
        </Button>
      </div>
    </form>
  </Card.Content>
</Card.Root>
