<script lang="ts">
  import { store } from './store.svelte';
  import Card from './Card.svelte';
  import { Trash2 } from 'lucide-svelte';
</script>

<div class="flex-1 w-full flex flex-col relative h-full">
  {#if store.items.length === 0}
    <div class="absolute inset-0 flex flex-col items-center justify-center text-stash-text/40 border-2 border-dashed border-stash-border rounded-xl">
      <span class="text-sm">Drop files here</span>
    </div>
  {:else}
    <div class="relative w-full h-[80px]"> <!-- Container height based on single card height -->
      {#each store.items as item, i (item.id)}
        <Card {item} index={i} total={store.items.length} />
      {/each}
    </div>
    
    <div class="absolute bottom-0 right-0 p-2">
      <button 
        class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium text-red-400 bg-red-400/10 hover:bg-red-400/20 rounded-lg transition-colors shadow-sm"
        onclick={() => store.clearAll()}
      >
        <Trash2 size={14} /> Clear All
      </button>
    </div>
  {/if}
</div>
