<script lang="ts">
  import { File, Link, Type, X, Shrink, Archive, Scissors, QrCode } from 'lucide-svelte';
  import type { DropPayload } from './store.svelte';
  import { store } from './store.svelte';
  import { startDrag } from '@crabnebula/tauri-plugin-drag';
  import { Spring } from 'svelte/motion';
  import { invoke } from '@tauri-apps/api/core';

  let { item, index, total }: { item: DropPayload, index: number, total: number } = $props();

  async function handleDragOut(event: MouseEvent) {
    if (item.item_type === 'file' || item.item_type === 'image' || item.item_type === 'zip' || item.item_type === 'qr') {
      try {
        await startDrag({
          item: [item.content],
          icon: ""
        });
      } catch (e) {
        console.error("Drag out failed", e);
      }
    }
  }

  function removeCard() {
    store.remove(item.id);
  }
  
  async function actionCompress() {
    try {
        const newPath = await invoke<string>('compress_image', { path: item.content });
        item.content = newPath;
        item.item_type = 'image';
    } catch(e) { console.error(e); }
  }

  async function actionZip() {
    try {
        const newPath = await invoke<string>('create_zip', { paths: [item.content] });
        item.content = newPath;
        item.item_type = 'zip';
    } catch(e) { console.error(e); }
  }

  async function actionCleanUrl() {
    try {
        const newUrl = await invoke<string>('clean_url', { urlStr: item.content });
        item.content = newUrl;
    } catch(e) { console.error(e); }
  }

  async function actionQr() {
    try {
        const newPath = await invoke<string>('generate_qr', { urlStr: item.content });
        // Add new QR card
        store.add({
            id: crypto.randomUUID(),
            item_type: 'qr',
            content: newPath,
            preview_path: newPath
        });
    } catch(e) { console.error(e); }
  }

  let zIndex = $derived(total - index);
  let opacity = $derived(1 - (index * 0.1));

  let translateY = new Spring(index * 12);
  let scale = new Spring(1 - (index * 0.05));

  $effect(() => {
    translateY.target = index * 12;
    scale.target = Math.max(0.8, 1 - (index * 0.05));
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  class="absolute w-full bg-stash-card rounded-xl shadow-lg border border-stash-border p-3 flex flex-col group transition-opacity duration-300 ease-out"
  style="z-index: {zIndex}; transform: translateY({translateY.current}px) scale({scale.current}); opacity: {opacity};"
>
  <div class="flex items-center justify-between cursor-grab active:cursor-grabbing" onmousedown={handleDragOut}>
    <div class="flex items-center gap-3 overflow-hidden">
      <div class="p-2 bg-stash-accent/20 text-stash-accent rounded-lg shrink-0">
        {#if item.item_type === 'file' || item.item_type === 'image' || item.item_type === 'qr'}
          <File size={20} />
        {:else if item.item_type === 'zip'}
          <Archive size={20} />
        {:else if item.item_type === 'url'}
          <Link size={20} />
        {:else}
          <Type size={20} />
        {/if}
      </div>
      
      <div class="truncate text-sm font-medium text-stash-text/90 pr-4">
        {item.item_type === 'file' || item.item_type === 'image' || item.item_type === 'zip' || item.item_type === 'qr' ? item.content.split('\\').pop()?.split('/').pop() : item.content}
      </div>
    </div>
    
    <div class="opacity-0 group-hover:opacity-100 flex gap-1 z-10 transition-opacity">
        <!-- Actions -->
        {#if item.item_type === 'image' || item.content.match(/\.(jpg|jpeg|png|webp)$/i)}
          <button class="p-1 text-stash-text/50 hover:text-stash-accent hover:bg-stash-accent/10 rounded-md transition-all" onclick={(e) => { e.stopPropagation(); actionCompress(); }} title="Compress Image"><Shrink size={16} /></button>
        {/if}
        {#if item.item_type === 'file' || item.item_type === 'image'}
          <button class="p-1 text-stash-text/50 hover:text-stash-accent hover:bg-stash-accent/10 rounded-md transition-all" onclick={(e) => { e.stopPropagation(); actionZip(); }} title="Zip File"><Archive size={16} /></button>
        {/if}
        {#if item.item_type === 'url'}
          <button class="p-1 text-stash-text/50 hover:text-stash-accent hover:bg-stash-accent/10 rounded-md transition-all" onclick={(e) => { e.stopPropagation(); actionCleanUrl(); }} title="Clean URL"><Scissors size={16} /></button>
          <button class="p-1 text-stash-text/50 hover:text-stash-accent hover:bg-stash-accent/10 rounded-md transition-all" onclick={(e) => { e.stopPropagation(); actionQr(); }} title="Generate QR"><QrCode size={16} /></button>
        {/if}
        
        <button 
          class="p-1 text-stash-text/50 hover:text-red-400 hover:bg-red-400/10 rounded-md transition-all shrink-0"
          onclick={(e) => { e.stopPropagation(); removeCard(); }}
          title="Remove"
        >
          <X size={16} />
        </button>
    </div>
  </div>
</div>
