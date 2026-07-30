export interface DropPayload {
    id: string;
    item_type: string;
    content: string;
    preview_path: string | null;
    timestamp: number;
}

export function createStore() {
    let items = $state<DropPayload[]>([]);

    // Auto-cleanup every minute
    if (typeof window !== 'undefined') {
        setInterval(() => {
            const now = Date.now();
            const fifteenMinutes = 15 * 60 * 1000;
            items = items.filter(item => now - item.timestamp < fifteenMinutes);
        }, 60000);
    }

    return {
        get items() { return items; },
        add(payload: Omit<DropPayload, 'timestamp'>) {
            if (!items.find(i => i.content === payload.content)) {
                items.push({ ...payload, timestamp: Date.now() });
            }
        },
        remove(id: string) {
            items = items.filter(i => i.id !== id);
        },
        clearAll() {
            items = [];
        }
    };
}

export const store = createStore();
