<script lang="ts">
  import { startDrag } from '@crabnebula/tauri-plugin-drag';

  let cards = $state<{id: string, text: string, type: 'file' | 'text'}[]>([
    { id: '1', text: 'Dummy File (Drag me out)', type: 'file' }
  ]);

  async function handleDragOut(event: MouseEvent, card: any) {
    if (card.type === 'file') {
      try {
        // Dummy file for drag out test. In reality, this would be the actual file path.
        await startDrag({
          item: ["/tmp/dummy.txt"],
          icon: ""
        });
      } catch (e) {
        console.error("Drag out failed", e);
      }
    }
  }
</script>

<div class="h-screen w-screen flex flex-col p-4 bg-transparent text-stash-text font-sans">
  <div class="w-full h-full bg-stash-bg/80 backdrop-blur-md rounded-2xl border border-stash-border shadow-2xl p-4 flex flex-col">
    <h1 class="text-xl font-semibold text-stash-accent mb-4 text-center">Stash</h1>
    
    <div class="flex-1 w-full border-2 border-dashed border-stash-border rounded-xl p-4 flex flex-col gap-3 overflow-y-auto">
      {#if cards.length === 0}
        <div class="flex h-full items-center justify-center text-stash-text/40">
          Drop files here
        </div>
      {/if}

      {#each cards as card}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div 
          class="bg-stash-card p-3 rounded-lg shadow-md cursor-grab active:cursor-grabbing border border-stash-border hover:border-stash-accent transition-colors flex items-center justify-between group"
          onmousedown={(e) => handleDragOut(e, card)}
        >
          <span class="truncate pr-4">{card.text}</span>
          <!-- Hover action placeholder -->
          <div class="opacity-0 group-hover:opacity-100 transition-opacity">
             <button class="text-xs bg-stash-accent/20 text-stash-accent px-2 py-1 rounded hover:bg-stash-accent/40">Action</button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
