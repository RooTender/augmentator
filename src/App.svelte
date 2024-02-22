<script lang="ts">
  import './styles.css'
  import Jumbotron from './lib/Jumbotron.svelte';
  import DirectorySettings from './lib/DirectorySettings.svelte';
  import TransformationsPanel from './lib/TransformationsPanel.svelte';
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
    <button type="button" class="btn btn-primary btn-lg w-100">Create augmented dataset</button>
  </div>
</main>