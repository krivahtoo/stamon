<script>
  import { Badge } from '$lib/components/ui/badge/index.js';

  /** @type {string[]} */
  export let tags = [];
</script>

<div class="flex flex-wrap gap-1">
  {#each tags as tag}
    <Badge variant="outline" class="font-normal">{tag}</Badge>
  {/each}
</div>
