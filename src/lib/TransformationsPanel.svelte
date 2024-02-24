<script lang="ts">
    import { advancedOptionsStore } from '../store/TransformationsStore';
    import { get } from 'svelte/store';

    type OptionType = 'everything' | 'paired' | 'custom';
    let selectedOption: OptionType = 'everything';
    
    const optionBehaviors = {
        'everything': () => get(advancedOptionsStore).map(option => ({ ...option, checked: true })),
        'paired': () => get(advancedOptionsStore).map(option => ({
            ...option,
            checked: [
                'hor_shift', 'ver_shift', 'rotate', 'mirror', 'flip', 
                'saturation', 'brightness', 'contrast', 'hue_rotation',
                'greyscale', 'invert'
            ].includes(option.id),
        })),
        'custom': () => get(advancedOptionsStore)
    };

    const radioOptions: { value: OptionType; label: string; }[] = [
        { value: 'everything', label: 'Everything' },
        { value: 'paired', label: 'Paired samples' },
        { value: 'custom', label: 'Custom' },
    ];

    function handleRadioChange() {
        advancedOptionsStore.set(optionBehaviors[selectedOption]());
    }

    function handleCheckboxChange(optionId: string, event: any) {
        advancedOptionsStore.update(options => options.map(option => {
            if (option.id === optionId) {
                return { ...option, checked: event.target.checked };
            }
            return option;
        }));
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
            {#each $advancedOptionsStore as option (option.id)}
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
    </div>
</div>
