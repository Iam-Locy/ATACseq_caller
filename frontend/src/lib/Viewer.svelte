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
                tracks: [],
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
                color: '#12432D',
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

<div id='viewer'>
    {#if loading}
        <h1>Loading peaks...</h1>
    {:else if error}
        <h1>Error: {error}</h1>
    {:else}
        <div>
            <h1>{sample.sample}</h1>
            <div id="stats">
                <p>Number of assigned reads: {sample.assigned_reads}</p>
                <p>Number of peaks: {sample.peaks}</p>
                <p>Fraction of reads in peaks: {sample.frip}</p>
            </div>
        </div>

    {/if}

    <div bind:this={containerEl} class="igv-container"></div>

</div>

<style>
    #viewer {
        padding: 10px;
        position: relative;
        top: 0px;
    }

    h1 {
        position: relative;
        left: 50%;
        transform: translateX(-50%);
        padding: 10px 20px;
        text-align: center;
        background-color: #12432D;
        width: fit-content;
        color: #FBF8EF;
        border-radius: 10px;
    }

    #stats{
        display: flex;
        justify-content: space-around;
       
    }

    #stats p{
        padding: 5px 10px;
        font-size: 20px;
        color: #FBF8EF;
        background-color: #12432D; 
        border-radius: 5px;
    }

    .igv-container {
        background-color: #fff !important;
        padding: 10px;
        margin: 20px 0px;
    }

    .igv-container :global(.igv-track-container),
    .igv-container:global(.igv-track),
    .igv-container :global(.igv-track .igv-canvas),
    .igv-container :global(.igv-sample-info-track),
    .igv-container:global(.igv-ruler) {
        background-color: transparent !important;
    }
</style>