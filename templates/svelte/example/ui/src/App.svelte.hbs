<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppClient } from '@holochain/client';
  import { AppWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import AllPosts from './forum/posts/AllPosts.svelte';
  import CreatePost from './forum/posts/CreatePost.svelte';

  import { clientContext } from './contexts';

  let client: AppClient | undefined;
  let loading = true;

  $: client, loading;

  onMount(async () => {
    client = await AppWebsocket.connect();
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <main>
      <h1>Forum</h1>

      <div id="content">
        <h2>All Posts</h2>
        <AllPosts style="margin-bottom: 16px" />
        <CreatePost />
      </div>
    </main>
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
