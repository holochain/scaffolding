<template>
  <div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else><div id="content"></div></div>
  </div>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppWebsocket, InstalledAppInfo } from '@holochain/client';
import '@material/mwc-circular-progress';

export default defineComponent({
  components: {
    // Add your subcomponents here
  },
  data(): { appWebsocket: AppWebsocket | undefined; loading: boolean; appInfo: InstalledAppInfo | undefined } {
    return {
      appWebsocket: undefined,
      loading: true,
      appInfo: undefined,
    };
  },
  async mounted() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    this.appWebsocket = await AppWebsocket.connect(`ws://localhost:${import.meta.env.VITE_HC_PORT}`);
    this.appInfo = await this.appWebsocket.appInfo({ installed_app_id: 'my-app' });

    this.loading = false;
  },
  provide() {
    return {
      appWebsocket: computed(() => this.appWebsocket),
      appInfo: computed(() => this.appInfo),
    };
  },
});
</script>