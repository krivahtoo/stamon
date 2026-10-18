<script>
  import { toast } from 'svelte-sonner';

  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import demo from '$lib/store/demo.js';
  import user from '$lib/store/user.js';
  import DemoCallout from './demo-callout.svelte';
  import { errorMessage, request, send } from './request.js';

  const MIN_PASSWORD_LEN = 8;
  const browserTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  /** @type {string[]} */
  const timezones = Intl.supportedValuesOf?.('timeZone') ?? [];

  // '' follows the browser.
  let timezone = $user?.timezone ?? '';
  let savingProfile = false;

  let current = '';
  let next = '';
  let confirm = '';
  let savingPassword = false;

  $: passwordError =
    next && next.length < MIN_PASSWORD_LEN
      ? `Use at least ${MIN_PASSWORD_LEN} characters.`
      : confirm && confirm !== next
        ? "The passwords don't match."
        : '';

  async function saveProfile() {
    savingProfile = true;
    try {
      await send('/user', 'PUT', { timezone: timezone || null });
      user.set((await request('/user')).user);
      toast.success('Timezone saved');
    } catch (e) {
      toast.error(errorMessage(e));
    } finally {
      savingProfile = false;
    }
  }

  async function changePassword() {
    if (passwordError || !current || !next) return;
    savingPassword = true;
    try {
      await send('/user/password', 'PUT', { current, new: next });
      current = next = confirm = '';
      toast.success('Password changed');
    } catch (e) {
      toast.error(errorMessage(e));
    } finally {
      savingPassword = false;
    }
  }
</script>

<svelte:head>
  <title>Account settings · Stamon</title>
</svelte:head>

<DemoCallout />

<section class="space-y-4 border-b pb-8">
  <div>
    <h3 class="text-lg font-semibold">1. Profile</h3>
    <p class="text-sm text-muted-foreground">Times on the dashboard are shown in this timezone.</p>
  </div>
  <dl class="grid grid-cols-[8rem_1fr] gap-y-2 text-sm">
    <dt class="text-muted-foreground">Username</dt>
    <dd class="font-medium">{$user?.username}</dd>
    <dt class="text-muted-foreground">Role</dt>
    <dd class="font-medium capitalize">{$user?.role}</dd>
  </dl>
  <form class="space-y-4" on:submit|preventDefault={saveProfile}>
    <div class="grid gap-2">
      <Label for="timezone">Timezone</Label>
      <select
        id="timezone"
        bind:value={timezone}
        disabled={$demo}
        class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
      >
        <option value="">Browser default ({browserTimezone})</option>
        {#each timezones as tz (tz)}
          <option value={tz}>{tz}</option>
        {/each}
      </select>
    </div>
    <div class="flex justify-end">
      <Button type="submit" disabled={$demo || savingProfile}>Save timezone</Button>
    </div>
  </form>
</section>

<section class="space-y-4 pt-8">
  <div>
    <h3 class="text-lg font-semibold">2. Password</h3>
    <p class="text-sm text-muted-foreground">You'll stay signed in on this device.</p>
  </div>
  <form class="space-y-4" on:submit|preventDefault={changePassword}>
    <div class="grid gap-2">
      <Label for="current-password">Current password</Label>
      <Input
        id="current-password"
        type="password"
        autocomplete="current-password"
        bind:value={current}
        disabled={$demo}
        required
      />
    </div>
    <div class="grid gap-2">
      <Label for="new-password">New password</Label>
      <Input
        id="new-password"
        type="password"
        autocomplete="new-password"
        bind:value={next}
        disabled={$demo}
        required
      />
    </div>
    <div class="grid gap-2">
      <Label for="confirm-password">Confirm new password</Label>
      <Input
        id="confirm-password"
        type="password"
        autocomplete="new-password"
        bind:value={confirm}
        disabled={$demo}
        required
      />
    </div>
    {#if passwordError}
      <p class="text-sm text-destructive">{passwordError}</p>
    {/if}
    <div class="flex justify-end">
      <Button
        type="submit"
        disabled={$demo || savingPassword || !!passwordError || !current || !next || !confirm}
      >
        Change password
      </Button>
    </div>
  </form>
</section>
