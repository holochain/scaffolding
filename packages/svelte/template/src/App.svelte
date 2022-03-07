<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { AppWebsocket, InstalledAppInfo } from '@holochain/client';
  import '@material/mwc-circular-progress';

  import { appWebsocketContext, appInfoContext } from './contexts';
  // TODO: Import the appropriate subcomponents

  let appWebsocket: AppWebsocket | undefined;
  let appInfo: InstalledAppInfo | undefined;
  let loading = true;
  let entryHash: string | undefined;

  $: appWebsocket, appInfo, entryHash, loading;

  onMount(async () => {
    appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

    appInfo = await appWebsocket.appInfo({
      installed_app_id: 'my-app',
    });
    loading = true;
  });

  setContext(appWebsocketContext, {
    getAppWebsocket: () => appWebsocket,
  });

  setContext(appInfoContext, {
    getAppInfo: () => appInfo,
  });
</script>

<main>
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <div id="content" />
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
