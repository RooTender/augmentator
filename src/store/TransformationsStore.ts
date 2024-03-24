import { writable } from 'svelte/store';

export const transformations = writable([
    { id: 'hor_shift', name: 'Horizontal shift*', checked: false },
    { id: 'ver_shift', name: 'Vertical shift*', checked: false },
    { id: 'crop', name: 'Crop', checked: false },
    { id: 'resize', name: 'Resize', checked: false },
    { id: 'rotate90', name: 'Rotate 90°', checked: false },
    { id: 'rotate180', name: 'Rotate 180°', checked: false },
    { id: 'rotate270', name: 'Rotate 270°', checked: false },
    { id: 'mirror', name: 'Mirror', checked: false },
    { id: 'flip', name: 'Flip', checked: false },
    { id: 'hue_rotation', name: 'Hue rotation', checked: false },
    { id: 'saturation', name: 'Saturation shift', checked: false },
    { id: 'brightness', name: 'Brightness shift', checked: false },
    { id: 'contrast', name: 'Contrast shift', checked: false },
    { id: 'grayscale', name: 'Grayscale', checked: false },
    { id: 'invert', name: 'Invert colors', checked: false},
    { id: 'color_norm', name: 'Color norm', checked: false },
]);
