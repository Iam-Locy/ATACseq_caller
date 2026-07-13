<script lang="ts">
    import type {File} from './types.ts'

    let {samples, onselect} : {samples: File[]; onselect: (sample: string) => void } = $props();
</script>

<nav>
    {#each samples as file, index (file.sample)}
        {@render menuCard(file)}
    {/each}
</nav>

{#snippet menuCard({sample}: {sample: string})}
    <button class="file" onclick={() => onselect(sample)}>{sample}</button>
{/snippet}

<style>
    nav {
        display: flex;
        flex-direction: column;
    }

    nav :global(.file) {
        background-color: #a3def3; /* flat light blue */
        border: solid 1px;
        border-radius: 0; /* sharp corners */
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
        transition: box-shadow 0.15s ease, transform 0.1s ease;
        box-shadow: none;
    }

    nav :global(.file):hover {
        box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.2); /* weak shadow */
    }

    nav :global(.file):active {
        box-shadow: 4px 4px 12px rgba(0, 0, 0, 0.4); /* stronger shadow on click */
        transform: translateY(1px);
    }

</style>