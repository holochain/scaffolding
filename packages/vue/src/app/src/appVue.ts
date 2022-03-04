import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const appVue = ({happName, appContent, appSubcomponents, subcomponentsImports}: {happName: string; appContent: string; appSubcomponents: string; subcomponentsImports: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `<template>
  <div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else>${appContent}</div>
  </div>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppWebsocket, InstalledAppInfo } from '@holochain/client';
import '@material/mwc-circular-progress';
${subcomponentsImports}

export default defineComponent({
  components: {
    ${appSubcomponents}
  },
  data(): {
    appWebsocket: AppWebsocket | undefined;
    loading: boolean;
    appInfo: InstalledAppInfo | undefined;
    entryHash: string | undefined;
  } {
    return {
      appWebsocket: undefined,
      loading: true,
      appInfo: undefined,
      entryHash: undefined,
    };
  },
  async mounted() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    this.appWebsocket = await AppWebsocket.connect(\`ws://localhost:\${import.meta.env.VITE_HC_PORT}\`);
    this.appInfo = await this.appWebsocket.appInfo({ installed_app_id: '${happName}' });

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
`
});
    