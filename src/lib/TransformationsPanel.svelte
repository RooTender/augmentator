<script lang="ts">
    type OptionType = 'everything' | 'paired' | 'custom';

    let selectedOption: OptionType = 'everything';
    let advancedOptions = [
        { id: 'rotate', name: 'Rotate', checked: false },
        { id: 'mirror', name: 'Mirror', checked: false },
        { id: 'flip', name: 'Flip', checked: false },
        { id: 'hsv_rotation', name: 'HSV rotation', checked: false },
        { id: 'hor_shift', name: 'Horizontal shift', checked: false },
        { id: 'ver_shift', name: 'Vertical shift', checked: false },
    ];

    const optionBehaviors = {
        'everything': () => advancedOptions.map(option => ({ ...option, checked: true })),
        'paired': () => advancedOptions.map(option => ({
            ...option,
            checked: option.id === 'rotate' || option.id === 'mirror',
        })),
        'custom': () => advancedOptions
    };

    const radioOptions: { value: OptionType; label: string; }[] = [
        { value: 'everything', label: 'Everything' },
        { value: 'paired', label: 'Paired samples' },
        { value: 'custom', label: 'Custom' },
    ];

    function handleRadioChange() {
        if (optionBehaviors[selectedOption]) {
            advancedOptions = optionBehaviors[selectedOption]();
        }
    }

    function handleCheckboxChange() {
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
            {#each advancedOptions as option, index (option.id)}
                <div class="col-sm-6 col-md-4 col-lg-3 col-xl-3">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id={option.id} bind:checked={option.checked} on:change={handleCheckboxChange}>
                        <label class="form-check-label" for={option.id}>
                            {option.name}
                        </label>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
