<script>
  import { onMount } from 'svelte';

  /** @type {number} */
  export const monitorId = 0;

  /** @type {number[]} */
  let statuses = [];
  // Define the tailwind classes for each status
  /** @type {string[]} */
  const statusClasses = [
    'bg-gray-500/30 dark:bg-gray-900/80',
    'bg-green-500 dark:bg-green-500/70',
    'bg-red-500 dark:bg-red-500/70',
    'bg-yellow-500'
  ];

  onMount(() => {
    // TODO: fetch recent statuses of this monitor
    statuses = [
      0, 0, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2
    ];
  });
</script>

<div class="flex space-x-1">
  {#each statuses as status, i}
    <div
      class={`hidden ${i > 20 ? 'sm:flex' : i > 10 ? 'md:flex' : 'lg:flex'} h-5 w-2 rounded-md ${statusClasses[status]}`}
    ></div>
  {/each}
</div>
