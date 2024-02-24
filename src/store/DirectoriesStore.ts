import { writable } from 'svelte/store';

export const directories = writable({
    input: '',
    target: '',
    output: '',
});
