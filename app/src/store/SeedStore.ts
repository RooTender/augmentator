import { writable } from 'svelte/store';

export const seed = writable<number>(1337);
