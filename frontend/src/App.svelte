<script lang="ts">
 import Sidebar from './lib/Sidebar.svelte';
 import Viewer from './lib/Viewer.svelte';
 import type {File} from './lib/types.ts';

 let samples = $state<File[]>([]);
 let selectedSample = $state<File | null>(null);
 let loading = $state(true);
 let error = $state<string | null>(null);

 $effect(() => {
    fetch('api/list')
        .then(r => {
            if (!r.ok) throw new Error(`HTTP ${r.status}`);
            return r.json();
        })
        .then(data => {samples = data; loading = false;})
        .catch(e => {error = e.message; loading = false;});
 });

 $inspect(samples)
 $inspect(selectedSample)
</script>

<div id="layout">
    {#if loading}
        <h1>Loading samples...</h1>
    {:else}
        <Sidebar {samples} onselect={(name) => {
            selectedSample = samples.find((sample) => sample.sample === name) ?? null;
        }}/>
    {/if}
  
    <main>
        {#if loading}
            <h1>Loading sample...</h1>
        {:else if error}
            <h1>{error}</h1>
        {:else}
            {#if selectedSample}
                <Viewer sample={selectedSample}/>
            {:else}
                <h1>Please select a sample!</h1>
            {/if}
        {/if}
    </main>
</div>


<style>

  #layout {
    display: grid;
    grid-template-columns: 20% 80%;
  }

</style>