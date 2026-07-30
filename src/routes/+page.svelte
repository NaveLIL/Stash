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
  
  let dialogType = $state<'none' | 'pin' | 'alert'>('none');
  let dialogMessage = $state('');
  let dialogInput = $state('');
  let dialogPeer = $state<any>(null);

  async function showPinPrompt(peer: any) {
    dialogType = 'pin';
    dialogPeer = peer;
    dialogInput = '';
    dialogMessage = `${t('enter_pin_for')} ${peer.name.split('.')[0]}`;
  }

  function showAlert(msg: string) {
    dialogType = 'alert';
    dialogMessage = msg;
  }

  function closeDialog() {
    dialogType = 'none';
  }

  async function submitPin() {
    if (dialogInput && dialogPeer) {
        const pin = dialogInput;
        const peer = dialogPeer;
        const item = store.items[0];
        closeDialog();
        try {
            await invoke('send_to_peer', { ip: peer.ip, port: peer.port, pin: pin, path: item.content });
            showAlert(t('sent_successfully'));
        } catch(e) {
            showAlert(`${t('failed')} ` + e);
        }
    }
  }

  async function testSendToPeer(peer: {name: string, ip: string, port: number}) {
      if (store.items.length > 0) {
         showPinPrompt(peer);
      } else {
         showAlert(t('no_items'));
      }
  }
</script>

<div class="h-screen w-screen flex flex-col p-4 bg-transparent text-stash-text font-sans relative">
  <div class="w-full h-full bg-stash-bg/80 backdrop-blur-md rounded-2xl border border-stash-border shadow-2xl p-4 flex flex-col overflow-hidden drag-region relative" style="--tauri-drag-region: true;">
    
    {#if dialogType !== 'none'}
      <div class="absolute inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
        <div class="bg-stash-bg border border-stash-border rounded-xl shadow-2xl p-5 w-full max-w-sm flex flex-col gap-4 transform transition-all">
          <h3 class="text-lg font-medium text-stash-text">{dialogMessage}</h3>
          
          {#if dialogType === 'pin'}
            <input type="text" bind:value={dialogInput} placeholder="----" class="w-full bg-stash-card border border-stash-border rounded-md px-3 py-2 text-stash-accent font-mono text-center text-xl focus:outline-none focus:ring-2 focus:ring-stash-accent" />
            <div class="flex gap-2 justify-end mt-2">
              <button class="px-4 py-2 text-sm text-stash-text/70 hover:bg-stash-card rounded-md transition-colors" onclick={closeDialog}>Cancel</button>
              <button class="px-4 py-2 text-sm bg-stash-accent text-white rounded-md hover:bg-stash-accent/80 transition-colors" onclick={submitPin}>Send</button>
            </div>
          {:else}
            <div class="flex justify-end mt-2">
              <button class="px-4 py-2 text-sm bg-stash-accent text-white rounded-md hover:bg-stash-accent/80 transition-colors" onclick={closeDialog}>OK</button>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <div class="flex justify-between items-center mb-4 relative z-10">
      <h1 class="text-xl font-semibold text-stash-accent pointer-events-none">Stash</h1>
      <div class="flex items-center gap-2 px-2 py-1 bg-stash-card rounded-md border border-stash-border text-sm">
         <Wifi size={14} class="text-green-400" />
         {t('pin')}: <span class="font-mono font-bold text-stash-accent">{myPin || '----'}</span>
      </div>
    </div>
    
    <div class="relative z-10 flex-1 overflow-hidden">
      <CardList />
    </div>
    
    {#if peers.length > 0}
        <div class="mt-4 pt-4 border-t border-stash-border overflow-y-auto max-h-[30vh] relative z-10">
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
