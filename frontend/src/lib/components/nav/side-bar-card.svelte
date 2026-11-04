<script>
  import * as Card from '$lib/components/ui/card/index.js';
  import { Button } from '$lib/components/ui/button/index.js';

  import { toast } from 'svelte-sonner';

  

  

  

  

  
  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {string} [description]
   * @property {string} [buttonText]
   * @property {string|undefined} [buttonHref]
   * @property {() => void} [callback]
   */

  /** @type {Props} */
  let {
    title = 'Upgrade to pro',
    description = 'Unlock all features and get unlimited access to our support team.',
    buttonText = 'Upgrade',
    buttonHref = undefined,
    callback = () => {
    toast.error('Event has been created', {
      description: 'Sunday, December 03, 2023 at 9:00 AM',
      action: {
        label: 'Undo',
        onClick: () => console.info('Undo')
      }
    });
  }
  } = $props();
</script>

<Card.Root
  data-x-chunk-name="dashboard-02-chunk-0"
  data-x-chunk-description="A card with a call to action"
>
  <Card.Header class="p-2 pt-0 md:p-4">
    <Card.Title>{title}</Card.Title>
    <Card.Description>
      {description}
    </Card.Description>
  </Card.Header>
  <Card.Content class="p-2 pt-0 md:p-4 md:pt-0">
    <Button size="sm" class="w-full" on:click={callback} href={buttonHref}>{buttonText}</Button>
  </Card.Content>
</Card.Root>
