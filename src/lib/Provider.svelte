<script lang="ts">

    import Tab from "./Tab.svelte";
    import ProviderBody from "./ProviderBody.svelte";

    export let id: String;
</script>

<div>
    <Tab/>
    <ProviderBody id={id} />
</div>
