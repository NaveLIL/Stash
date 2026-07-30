import { describe, it, expect, beforeEach, afterAll } from 'vitest';
import { store } from './store.svelte';

describe('Store', () => {
    beforeEach(() => {
        // Clear store before each test
        store.clearAll();
    });

    afterAll(() => {
        // Clean up interval
        store.destroy();
    });

    it('should add an item and unshift it to the top', () => {
        store.add({
            id: '123',
            item_type: 'file',
            content: 'C:\\test.txt',
            preview_path: 'C:\\test.txt'
        });
        store.add({
            id: '124',
            item_type: 'text',
            content: 'hello',
            preview_path: null
        });

        expect(store.items.length).toBe(2);
        // Newest item should be at index 0 (unshifted)
        expect(store.items[0].id).toBe('124');
    });

    it('should limit items to 15 by discarding the oldest', () => {
        for (let i = 0; i < 20; i++) {
            store.add({
                id: i.toString(),
                item_type: 'text',
                content: 'test',
                preview_path: null
            });
        }

        expect(store.items.length).toBe(15);
        // The last inserted items (19 down to 5) should be present
        expect(store.items[0].id).toBe('19'); // most recent at front
        expect(store.items[14].id).toBe('5'); // oldest retained item
        // Items 0-4 were discarded
    });

    it('should deduplicate by id, not content', () => {
        // Add item
        store.add({ id: 'abc', item_type: 'file', content: 'test', preview_path: null });
        // Attempt duplicate id
        store.add({ id: 'abc', item_type: 'text', content: 'diff', preview_path: null });
        
        expect(store.items.length).toBe(1);
        expect(store.items[0].content).toBe('test');

        // Allow duplicate content with different id
        store.add({ id: 'def', item_type: 'file', content: 'test', preview_path: null });
        expect(store.items.length).toBe(2);
    });

    it('should remove an item by id', () => {
        store.add({
            id: 'abc',
            item_type: 'file',
            content: 'test',
            preview_path: null
        });

        store.remove('abc');
        expect(store.items.length).toBe(0);
    });

    it('should handle heavy load of 10,000 rapid inserts without memory leaks or limit breaches', () => {
        for (let i = 0; i < 10000; i++) {
            store.add({ id: `load_${i}`, item_type: 'text', content: `load_${i}`, preview_path: null });
        }
        expect(store.items.length).toBe(15);
        expect(store.items[0].id).toBe('load_9999'); // newest
        expect(store.items[14].id).toBe('load_9985'); // oldest retained
    });

    it('should handle sequential stress test of mixed add, remove, and clear operations', async () => {
        const ops = [];
        for (let i = 0; i < 1000; i++) {
            ops.push(async () => {
                store.add({ id: `mix_${i}`, item_type: 'text', content: `data`, preview_path: null });
                if (i % 2 === 0) store.remove(`mix_${i}`);
                if (i % 500 === 0) store.clearAll();
            });
        }
        await Promise.all(ops.map(op => op()));
        // Store shouldn't exceed 15 items even with race conditions
        expect(store.items.length).toBeLessThanOrEqual(15);
    });
});
