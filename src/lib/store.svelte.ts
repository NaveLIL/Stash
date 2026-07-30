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

    // Auto-cleanup every minute
    if (typeof window !== 'undefined') {
        intervalId = setInterval(() => {
            const now = Date.now();
            const fifteenMinutes = 15 * 60 * 1000;
            items = items.filter(item => now - item.timestamp < fifteenMinutes);
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
                    items.pop(); // Discard oldest
                }
            }
        },
        remove(id: string) {
            items = items.filter(i => i.id !== id);
        },
        clearAll() {
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
