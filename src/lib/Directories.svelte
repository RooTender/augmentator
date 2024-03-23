<script lang="ts">
  import { dialog } from '@tauri-apps/api';
  import { directories } from '../store/DirectoriesStore';

  let displayedDirs = {
    input: '',
    output: '',
  };

  async function selectDirectory(type: 'input' | 'output') {
    try {
      const selected = await dialog.open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        const fullPath = selected.toString();
        directories.update(currentDirs => {
          currentDirs[type] = fullPath;
          return currentDirs;
        });
        displayedDirs[type] = formatDirectoryPath(fullPath);
      } 
    } catch (error) {
      console.error('Error selecting directory:', error);
    }
  }

  function formatDirectoryPath(fullPath: string): string {
    const parts = fullPath.split(/[/\\]/);
    
    if (parts.length > 1) {
      return `.../${parts.slice(-2).join('/')}`;
    } else {
      return fullPath;
    }
  }
</script>

<h2>Directories</h2>
<div class="row">
    <div class="col">
        <div class="input-group mb-5">
            <div class="input-group-prepend">
              <span class="input-group-text" id="basic-addon1">📥</span>
            </div>
            <input
                bind:value={displayedDirs.input}
                on:click|preventDefault={() => selectDirectory('input')}
                type="text" class="form-control" placeholder="Input images" 
                aria-label="inputs_dir" aria-describedby="basic-addon1">
        </div>
    </div>
    <div class="col">
        <div class="input-group mb-5">
            <div class="input-group-prepend">
              <span class="input-group-text" id="basic-addon1">📤</span>
            </div>
            <input 
                bind:value={displayedDirs.output}
                on:click|preventDefault={() => selectDirectory('output')}
                type="text" class="form-control" placeholder="Augmentation output" 
                aria-label="result_dir" aria-describedby="basic-addon1">
        </div>
    </div>
</div>
