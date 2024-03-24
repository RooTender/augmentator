<script lang="ts">
    import { transformations } from '../store/TransformationsStore';
    import { get } from 'svelte/store';

    type OptionType = 'everything' | 'preserve_colors' | 'preserve_shape' | 'custom';
    let selectedOption: OptionType = 'everything';
    
    const optionBehaviors = {
        'everything': () => get(transformations).map(option => ({ ...option, checked: true })),
        'preserve_colors': () => get(transformations).map(option => ({
            ...option,
            checked: [
                'hor_shift', 'ver_shift',
                'rotate90', 'rotate180', 'rotate270',
                'mirror', 'flip'
            ].includes(option.id),
        })),
        'preserve_shape': () => get(transformations).map(option => ({
            ...option,
            checked: [
                'hue_rotation', 'saturation', 'brightness',
                'contrast', 'grayscale', 'invert', 'color_norm'
            ].includes(option.id),
        })),
        'custom': () => get(transformations)
    };

    const radioOptions: { value: OptionType; label: string; }[] = [
        { value: 'everything', label: 'Everything' },
        { value: 'preserve_colors', label: 'Preserve colors' },
        { value: 'preserve_shape', label: 'Preserve shape' },
        { value: 'custom', label: 'Custom' },
    ];

    function handleRadioChange() {
        transformations.set(optionBehaviors[selectedOption]());
    }

    function handleCheckboxChange(optionId: string, event: any) {
        transformations.update(options => options.map(option => {
            if (option.id === optionId) {
                return { ...option, checked: event.target.checked };
            }
            return option;
        }));
        selectedOption = 'custom';
    }

    handleRadioChange();
</script>

<h2>Transformations</h2>
<div class="row">
    <div class="col-4">
        <h5>Basic</h5>
        <div class="list-group list-group-flush">
            {#each radioOptions as {value, label}}
                <label class="list-group-item">
                    <input type="radio" class="form-check-input me-1" value={value} bind:group={selectedOption} on:change={handleRadioChange}>
                    {label}
                </label>
            {/each}
        </div>
    </div>
    <div class="col">
        <h5>Advanced</h5>
        <div class="row">
            {#each $transformations as option (option.id)}
                <div class="col-sm-6 col-md-4 col-lg-3 col-xl-3">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id={option.id} 
                            bind:checked={option.checked}
                            on:change={(event) => handleCheckboxChange(option.id, event)}>
                        <label class="form-check-label" for={option.id}>
                            {option.name}
                        </label>
                    </div>
                </div>
            {/each}
        </div>
        <p>*<i>Applied to every transformation.</i></p>
    </div>
</div>
