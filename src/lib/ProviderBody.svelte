<script type="ts">
    import type {UpdateStatus} from "./updateStatus";
    import {getProvider, Provider} from "./providers";

    export let id: String = "";

    let provider: Provider = getProvider(id);
    $: provider = getProvider(id);
    let list: UpdateStatus[] = [];

    async function checkUpdate() {
        list = await provider.checkUpdate();
    }

    checkUpdate()

</script>

<div>
    <table>
        <tbody>
            <tr>
                <th>name</th>
                <th>current version</th>
                <th>new version</th>
            </tr>
            {#each list as item}
                <tr>
                    <td>{item.name}</td>
                    <td>{item.currentVersion}</td>
                    <td>{item.newVersion}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>
