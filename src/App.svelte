<script lang="ts">

  import Router from 'svelte-spa-router'
  import Main from "./Main.svelte";
  import Scoop from "./provider/Scoop.svelte";

  const routes = {
    '/': Main,
    '/provider/scoop': Scoop
  };

</script>

<main>
  <Router routes={routes}></Router>
</main>

