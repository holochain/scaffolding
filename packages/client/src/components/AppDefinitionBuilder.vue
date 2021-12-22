<template>
  <div class="column" style="flex: 1; margin: 16px">
    <span style="font-size: 24px">Scaffold New App</span>

    <happ-definition-builder
      style="flex: 1; margin: 16px; display: flex"
      ref="defineHapp"
      :uiTemplates="UiTemplates"
      @happ-changed="happ = $event.detail[0]"
    >
    </happ-definition-builder>

    <mwc-fab
      @click="requestScaffold()"
      style="--mdc-theme-secondary: #4720e3; position: absolute; right: 16px; bottom: 16px"
      label="Scaffold app"
      :disabled="!happ"
      extended
      icon="system_update_alt"
    ></mwc-fab>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import { newDna } from '@holochain/rad-elements';
import type { Select } from '@material/mwc-select';
import { UiTemplates } from '../types';

export default defineComponent({
  name: 'AppDefinitionBuilder',

  data(): {
    happ: HappDefinition;
    UiTemplates: string[];
  } {
    return {
      UiTemplates,
      happ: {
        name: 'my-app',
        dnas: [newDna()],
      },
    };
  },
  methods: {
    requestScaffold() {
      const uiTemplate = (this.$refs.uiTemplateSelect as Select).value;

      this.$emit('scaffoldApp', { happ: this.happ, uiTemplate });
    },
  },
  emits: ['scaffoldApp'],
});
</script>
<style scoped>
.text-input {
  width: 424px;
}
</style>