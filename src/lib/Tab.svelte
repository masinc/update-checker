<script type="ts">
    import {push} from 'svelte-spa-router'
    import {providers} from "./providers";

</script>

<div class="tab-container">
    {#each providers as provider}
        <div class="tab" on:click = {() => push(provider.getPath())}>
            <span>{provider.caption}</span>
        </div>
    {/each}
</div>

<style>
    .tab-container {
        display: flex;
        border: #0f0f0f outset 1px;
    }

    .tab:hover {
        background-color: #999;
    }
</style>
