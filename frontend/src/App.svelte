<script lang="ts">
 import Sidebar from './lib/Sidebar.svelte';
 import Viewer from './lib/Viewer.svelte';
 import type {File} from './lib/types.ts';

 let samples = $state<File[]>([]);
 let selectedSample = $state<File | null>(null);
 let loading = $state(true);
 let error = $state<string | null>(null);

 $effect(() => {
    loading = true;
    error = null;
    
    fetch('api/list')
        .then(r => {
            if (!r.ok) throw new Error(`HTTP ${r.status}`);
            return r.json();
        })
        .then(data => {samples = data; loading = false;})
        .catch(e => {error = e.message; loading = false;});
 });

</script>

<div id="layout">

    {#if error}
        <h1>Error: {error}</h1>
    {:else}
        <Sidebar isloading={loading} {samples} onselect={(name) => {
                selectedSample = samples.find((sample) => sample.sample === name) ?? null;
        }}/>
    {/if}
  
    <main>
        {#if selectedSample}
            <Viewer sample={selectedSample}/>
        {:else}
            <h1>Please select a sample!</h1>
        {/if}
    </main>
</div>


<style>
    #layout {
        height: 100%;
        display: grid;
        grid-template-columns: 20% 80%;
    }

    main{
        height: 80%;
        margin: 5px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        background-color: #FBF8EF;
        border-radius: 20px;
    }

    main h1{
        text-align: center;
        color: #12432D; 
    }

</style>