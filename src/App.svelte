<script lang="ts">
    import './styles.css'
    import Jumbotron from './lib/Jumbotron.svelte';
    import DirectorySettings from './lib/DirectorySettings.svelte';
    import TransformationsPanel from './lib/TransformationsPanel.svelte';

    import { advancedOptionsStore } from './store/TransformationsStore';
    import { invoke } from '@tauri-apps/api/tauri';
    import { get } from 'svelte/store';

    async function createAugmentedDataset() {
        const selectedOptions = get(advancedOptionsStore)
            .filter(option => option.checked)
            .map(option => option.id);
        
        try {
            const result = await invoke('your_tauri_command', { options: selectedOptions });
            console.log(result);
        } catch (error) {
            console.error('Failed to create augmented dataset:', error);
        }
    }
</script>

<Jumbotron/>
<hr>
<main class="container">
  <DirectorySettings/>
  <TransformationsPanel/>
  <h2>Submit</h2>
  <div class="row text-center lead">
      <p>This will enlarge dataset
      <b>3 times</b> resulting with X images!</p>
  </div>
  <div class="row">
    <button type="button" class="btn btn-primary btn-lg w-100" on:click={createAugmentedDataset}>
        Create augmented dataset
    </button>
  </div>
</main>