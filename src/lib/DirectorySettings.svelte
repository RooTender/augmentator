<script lang="ts">
  import { dialog } from '@tauri-apps/api';

  let dirs = {
    input: '',
    target: '',
    output: '',
  };

  let displayedDirs = {
    input: '',
    target: '',
    output: '',
  };

  enum dirType {
    input = 'input',
    target = 'target',
    output = 'output',
  }

  async function selectDirectory(type: keyof typeof dirs) {
    try {
      const selected = await dialog.open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        const fullPath = selected.toString();
        dirs[type] = fullPath;
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
        <div class="input-group mb-3">
            <div class="input-group-prepend">
              <span class="input-group-text" id="basic-addon1">📥</span>
            </div>
            <input
                bind:value={displayedDirs.input}
                on:click|preventDefault={() => selectDirectory(dirType.input)}
                type="text" class="form-control" placeholder="Input images" 
                aria-label="inputs_dir" aria-describedby="basic-addon1">
        </div>
    </div>
    <div class="col">
        <div class="input-group mb-3">
            <div class="input-group-prepend">
              <span class="input-group-text" id="basic-addon1">🎯</span>
            </div>
            <input 
                bind:value={displayedDirs.target}
                on:click|preventDefault={() => selectDirectory(dirType.target)}
                type="text" class="form-control" placeholder="Targets images" 
                aria-label="targets_dir" aria-describedby="basic-addon1">
        </div>
    </div>
    <div class="col">
        <div class="input-group mb-3">
            <div class="input-group-prepend">
              <span class="input-group-text" id="basic-addon1">📤</span>
            </div>
            <input 
                bind:value={displayedDirs.output}
                on:click|preventDefault={() => selectDirectory(dirType.output)}
                type="text" class="form-control" placeholder="Augmentation output" 
                aria-label="result_dir" aria-describedby="basic-addon1">
        </div>
    </div>
</div>
