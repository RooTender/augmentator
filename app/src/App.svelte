<script lang="ts">
    import './styles.css'
    import Jumbotron from './lib/Jumbotron.svelte';
    import Directories from './lib/Directories.svelte';
    import Transformations from './lib/Transformations.svelte';

    import { transformations } from './store/TransformationsStore';
    import { directories } from './store/DirectoriesStore';
    import { invoke } from '@tauri-apps/api/core';
    import { get } from 'svelte/store';

    let errorMessage: any;

    async function createAugmentedDataset() {
        const selectedTransformations = get(transformations)
            .filter(option => option.checked)
            .map(option => option.id);
        const selectedDirectories = get(directories);
        
        try {
            const result = await invoke('augment_dataset', {
                directories: selectedDirectories,
                transformations: selectedTransformations
            });
            console.log(result);
            errorMessage = "";
        } catch (error) {
            console.error('Failed to create augmented dataset:', error);
            errorMessage = error;
        }
    }
</script>

<Jumbotron/>
<hr>
<main class="container">
  <Directories/>
  <Transformations/>
  <h2>Submit</h2>
  {#if errorMessage}
    <div class="alert alert-danger" role="alert">{errorMessage}</div>
  {/if}
  
  <div class="row">
    <button type="button" class="btn btn-primary btn-lg w-100 mb-3" on:click={createAugmentedDataset}>
      Create augmented dataset
    </button>
  </div>
</main>