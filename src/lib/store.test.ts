import { describe, it, expect, beforeEach } from 'vitest';
import { store } from './store.svelte';

describe('Store', () => {
    beforeEach(() => {
        // Clear store before each test
        store.items.forEach(item => store.remove(item.id));
    });

    it('should add an item', () => {
        store.add({
            id: '123',
            item_type: 'file',
            content: 'C:\\test.txt',
            preview_path: 'C:\\test.txt'
        });

        expect(store.items.length).toBe(1);
        expect(store.items[0].id).toBe('123');
    });

    it('should limit items to 15', () => {
        for (let i = 0; i < 20; i++) {
            store.add({
                id: i.toString(),
                item_type: 'text',
                content: 'test',
                preview_path: null
            });
        }

        expect(store.items.length).toBe(15);
        // The first 5 items should have been dropped (0, 1, 2, 3, 4)
        expect(store.items[0].id).toBe('19'); // most recent at front
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
});
