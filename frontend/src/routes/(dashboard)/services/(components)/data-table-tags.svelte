<script>
  import { Badge } from '$lib/components/ui/badge/index.js';

  
  /**
   * @typedef {Object} Props
   * @property {string[]} [tags]
   */

  /** @type {Props} */
  let { tags = [] } = $props();
</script>

<div class="flex flex-wrap gap-1">
  {#each tags as tag}
    <Badge variant="outline" class="font-normal">{tag}</Badge>
  {/each}
</div>
