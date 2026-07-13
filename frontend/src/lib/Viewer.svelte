<script lang="ts">
    import type {File, Peak} from './types.ts';

    let {sample} : {sample: File} = $props();

    let peaks = $state<Peak[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    $effect(() => {
        loading = true;
        error = null;

        fetch(`/api/peaks/${encodeURIComponent(sample.sample)}`)
            .then(r => {
                if (!r.ok) throw new Error(`HTTP ${r.status}`);
                return r.json();
            })
            .then(data => { peaks = data; loading = false; })
            .catch(e => { error = e.message; loading = false; });
    });

    $inspect(peaks);
</script>

{#if loading}
    <h1>Loading peaks...</h1>
{:else if error}
    <h1>Error: {error}</h1>
{:else}
    <div>
        <h1>{sample.sample}</h1>
        <p>Number of assigned reads: {sample.assigned_reads}</p>
        <p>Number of peaks: {sample.peaks}</p>
        <p>Fraction of reads in peaks: {sample.frip}</p>
    </div>

{/if}