import { invoke } from '@tauri-apps/api/core';

export interface DropPayload {
    id: string;
    item_type: string;
    content: string;
    preview_path: string | null;
    timestamp: number;
}

export function createStore() {
    let items = $state<DropPayload[]>([]);
    const MAX_ITEMS = 15;
    let intervalId: ReturnType<typeof setInterval>;

    function cleanupItem(item: DropPayload) {
        if (typeof window !== 'undefined' && (item.item_type === 'file' || item.item_type === 'image' || item.item_type === 'zip' || item.item_type === 'qr')) {
            invoke('cleanup_temp_file', { path: item.content }).catch(console.error);
        }
    }

    // Auto-cleanup every minute
    if (typeof window !== 'undefined') {
        intervalId = setInterval(() => {
            const now = Date.now();
            const fifteenMinutes = 15 * 60 * 1000;
            const kept: DropPayload[] = [];
            for (const item of items) {
                if (now - item.timestamp < fifteenMinutes) {
                    kept.push(item);
                } else {
                    cleanupItem(item);
                }
            }
            items = kept;
        }, 60000);
    }

    return {
        get items() { return items; },
        add(payload: Omit<DropPayload, 'timestamp'>) {
            // Deduplicate by id, not content
            if (!items.find(i => i.id === payload.id)) {
                // Newest item first
                items.unshift({ ...payload, timestamp: Date.now() });
                // Enforce hard limit
                if (items.length > MAX_ITEMS) {
                    const discarded = items.pop(); // Discard oldest
                    if (discarded) cleanupItem(discarded);
                }
            }
        },
        remove(id: string) {
            const toRemove = items.find(i => i.id === id);
            if (toRemove) cleanupItem(toRemove);
            items = items.filter(i => i.id !== id);
        },
        clearAll() {
            for (const item of items) {
                cleanupItem(item);
            }
            items = [];
        },
        destroy() {
            if (intervalId) {
                clearInterval(intervalId);
            }
        }
    };
}

export const store = createStore();
