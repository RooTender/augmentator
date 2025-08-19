<script lang="ts">
    import './styles.css'
    import Jumbotron from './lib/Jumbotron.svelte';
    import Directories from './lib/Directories.svelte';
    import Transformations from './lib/Transformations.svelte';

    import { transformations } from './store/TransformationsStore';
    import { directories } from './store/DirectoriesStore';
    import { seed } from './store/SeedStore';
    import { invoke } from '@tauri-apps/api/core';
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
    import { get } from 'svelte/store';
    import { onMount } from 'svelte';
  
  let errorMessage: string | null = null;
  let isAugmenting = false;
  let percent = 0;

  onMount(() => {
    const win = getCurrentWebviewWindow();
    let unsubs: Array<() => void> = [];

    (async () => {
      unsubs.push(
        await win.listen<number>('augment-started', () => {
          isAugmenting = true;
          percent = 0;
        }),
        await win.listen<{ processed:number; total:number; percent:number }>('augment-progress', (e) => {
          percent = e.payload.percent ?? Math.round((e.payload.processed / e.payload.total) * 100);
        }),
        await win.listen('augment-finished', () => {
          percent = 100;
          isAugmenting = false;
        }),
        await win.listen<string>('augment-error', (e) => {
          errorMessage = e.payload || 'Augmentation failed.';
          isAugmenting = false;
        }),
      );
    })();

    return () => { unsubs.forEach(u => u()); };
  });

  async function createAugmentedDataset() {
    errorMessage = null;

    const selectedTransformations = get(transformations).filter(o => o.checked).map(o => o.id);
    const selectedDirectories = get(directories);
    const baseSeed = Number(get(seed)) ?? 0;

    try {
      // komenda zwróci szybko; progres idzie eventami
      await invoke('augment_dataset', {
        directories: selectedDirectories,
        transformations: selectedTransformations,
        seed: baseSeed,
      });
    } catch (e) {
      errorMessage = String(e);
      isAugmenting = false;
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
    <button 
      type="button" 
      class="btn btn-primary btn-lg w-100 mb-3" 
      on:click={createAugmentedDataset}
      disabled={isAugmenting}
    >
      {#if isAugmenting}
        Augmenting ({percent}%)
      {:else}
        Create augmented dataset
      {/if}
    </button>
  </div>
</main>