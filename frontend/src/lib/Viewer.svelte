<script lang="ts">
    import igv from 'igv';
    import type {File, Peak} from './types.ts';

    let {sample} : {sample: File} = $props();

    let peaks = $state<Peak[]>([]);
    let peakTrack: any = null;
    let loading = $state(true);
    let error = $state<string | null>(null);
    let containerEl: HTMLDivElement | undefined = $state();
    let browser: any = $state(null);

    $effect(() => {
        if (!containerEl) return;

        (async () => {
            browser = await igv.createBrowser(containerEl, {
                genome: 'danRer11',
                locus: 'chr3',
                tracks: []
            });

        })();

       

        return () => {
            if (browser) igv.removeBrowser(browser);
        }
    });

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


    $effect(() => {
        if (!browser || peaks.length === 0) return;

        (async () => {
            if (peakTrack) {
                browser.removeTrack(peakTrack);
            }

            peakTrack = await browser.loadTrack({
                name: `${sample.sample} peaks`,
                type: "annotation",
                format: "narrowPeak",
                features: peaks.map(p => ({
                    chr: p.chrom,
                    start: p.start,
                    end: p.end,
                    name: p.name,
                    score: p.score
                }))
            });
        })().catch(console.error);
    });

</script>

<div>
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

    <div bind:this={containerEl} class="igv-container"></div>

</div>

<style>
    .igv-container {
        width: 100%;
        height: 600px;
    }
</style>