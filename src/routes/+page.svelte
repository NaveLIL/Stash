<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow, currentMonitor, LogicalPosition } from '@tauri-apps/api/window';
  import { register } from '@tauri-apps/plugin-global-shortcut';
  import { invoke } from '@tauri-apps/api/core';
  import CardList from '$lib/CardList.svelte';
  import { store, type DropPayload } from '$lib/store.svelte';
  import { Wifi, Send } from 'lucide-svelte';
  import { t } from '$lib/i18n.svelte';

  let myPin = $state<number>(0);
  let peers = $state<{name: string, ip: string, port: number}[]>([]);

  onMount(() => {
    const appWindow = getCurrentWindow();

    const unlistenPin = listen<number>('stash://pin', (event) => {
      myPin = event.payload;
    });

    const unlistenPeerFound = listen<{name: string, ip: string, port: number}>('stash://peer-found', (event) => {
      if (!peers.find(p => p.name === event.payload.name)) {
        peers.push(event.payload);
      }
    });

    const unlistenPeerLost = listen<string>('stash://peer-lost', (event) => {
      peers = peers.filter(p => p.name !== event.payload);
    });

    const unlistenFocus = appWindow.onFocusChanged(async ({ payload: focused }) => {
      if (focused) {
        const monitor = await currentMonitor();
        if (monitor) {
          const factor = monitor.scaleFactor;
          const physicalSize = await appWindow.innerSize();
          const logicalSize = physicalSize.toLogical(factor);
          const logicalMonitorSize = monitor.size.toLogical(factor);
          const x = logicalMonitorSize.width - logicalSize.width - 20;
          const y = (logicalMonitorSize.height - logicalSize.height) / 2;
          await appWindow.setPosition(new LogicalPosition(x, y));
        }
      }
    });

    register('CommandOrControl+Shift+Space', async (e) => {
      if (e.state === 'Pressed') {
        const isVisible = await appWindow.isVisible();
        if (isVisible) {
          await appWindow.hide();
        } else {
          await appWindow.show();
          await appWindow.setFocus();
        }
      }
    });

    const unlistenStash = listen<DropPayload>('stash://item-dropped', (event) => {
      store.add({
        id: event.payload.id,
        item_type: event.payload.item_type,
        content: event.payload.content,
        preview_path: event.payload.preview_path,
      });
      appWindow.show();
      appWindow.setFocus();
    });
    
    const unlistenFileDrop = listen<{ paths: string[] }>('tauri://drag-drop', (event) => {
      // Windows uses OLE (stash://item-dropped). Only use Tauri's drop on macOS/Linux to avoid duplicates.
      if (!navigator.userAgent.includes('Windows')) {
        event.payload.paths.forEach(path => {
          store.add({
            id: crypto.randomUUID(),
            item_type: 'file',
            content: path,
            preview_path: path
          });
        });
        appWindow.show();
        appWindow.setFocus();
      }
    });

    return () => {
      unlistenStash.then(f => f());
      unlistenFileDrop.then(f => f());
      unlistenFocus.then(f => f());
      unlistenPin.then(f => f());
      unlistenPeerFound.then(f => f());
      unlistenPeerLost.then(f => f());
    };
  });
  
  async function testSendToPeer(peer: any) {
     if (store.items.length > 0) {
         let item = store.items[0];
         let pin = prompt(`${t('enter_pin')} ${peer.name}`);
         if (pin) {
             try {
                 await invoke('send_to_peer', { ip: peer.ip, port: peer.port, pin: pin, path: item.content });
                 alert(t('sent_successfully'));
             } catch(e) { alert(`${t('failed')} ` + e); }
         }
     } else {
         alert(t('no_items'));
     }
  }
</script>

<div class="h-screen w-screen flex flex-col p-4 bg-transparent text-stash-text font-sans">
  <div class="w-full h-full bg-stash-bg/80 backdrop-blur-md rounded-2xl border border-stash-border shadow-2xl p-4 flex flex-col overflow-hidden drag-region" style="--tauri-drag-region: true;">
    
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-xl font-semibold text-stash-accent pointer-events-none">Stash</h1>
      <div class="flex items-center gap-2 px-2 py-1 bg-stash-card rounded-md border border-stash-border text-sm">
         <Wifi size={14} class="text-green-400" />
         {t('pin')}: <span class="font-mono font-bold text-stash-accent">{myPin || '----'}</span>
      </div>
    </div>
    
    <CardList />
    
    {#if peers.length > 0}
        <div class="mt-4 pt-4 border-t border-stash-border overflow-y-auto max-h-[30vh]">
            <h2 class="text-xs font-semibold text-stash-text/60 mb-2 uppercase tracking-wider">{t('nearby_devices')}</h2>
            <div class="flex flex-col gap-2">
                {#each peers as peer}
                    <div class="flex items-center justify-between bg-stash-card p-2 rounded-lg border border-stash-border">
                        <span class="text-sm truncate">{peer.name.split('.')[0]}</span>
                        <button class="p-1.5 bg-stash-accent/20 text-stash-accent rounded-md hover:bg-stash-accent/40 transition-colors" onclick={() => testSendToPeer(peer)} title={t('send_top_item')}>
                            <Send size={14} />
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
  </div>
</div>
