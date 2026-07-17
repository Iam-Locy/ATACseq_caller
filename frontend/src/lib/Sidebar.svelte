<script lang="ts">
    import type {File} from './types.ts'

    let {isloading, samples, onselect} : {isloading: boolean; samples: File[]; onselect: (sample: string) => void } = $props();
</script>

<nav>
    {#if isloading}

        <h1>Loading ...</h1>

    {:else}

        {#each samples as file, index (file.sample)}
            {@render menuCard(file)}
        {/each}

    {/if}
</nav>

{#snippet menuCard({sample}: {sample: string})}
    <button class="file" onclick={() => onselect(sample)}>{sample}</button>
{/snippet}

<style>
    nav {
        display: flex;
        flex-direction: column;
        background-color: #FBF8EF;
        padding: 20px 0px;
        border-radius: 20px;
        margin: 5px;
    }

    nav h1{
        color: #12432D;
        text-align: center;
    }

    nav :global(.file) {
        position: relative;
        border-radius: 0; /* sharp corners */
        padding: 10px 20px;
        font-size: 20px;
        font-family: Arial, Helvetica, sans-serif;
        font-weight: bold;
        color: #12432D;
        cursor: pointer;
        transition: box-shadow 0.15s ease, transform 0.1s ease;
        background-color: #FBF8EF;
        box-shadow: none;
        border: none;
    }

    nav :global(.file)::after{
        content: "";
        position: absolute;
        bottom: 0px;
        left: 10%;
        width: 80%;       /* however narrow you want */
        height: 1px;
        background: #12432d97;
    }

    nav :global(.file):hover {
        color: #368461;
    }

</style>