<template>
  <div class="column" style="flex: 1; margin: 16px">
    <webhapp-definition-builder
      style="flex: 1; display: flex"
      ref="defineHapp"
      :uitemplates="UiTemplates.join(',')"
      @happ-changed="happ = $event.detail[0]"
    >
    </webhapp-definition-builder>

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
import { WebHappDefinition } from '@holochain/rad-definitions';
import { newHappDef } from '@holochain/rad-elements';
import { UiTemplates } from '../types';

export default defineComponent({
  name: 'AppDefinitionBuilder',

  data(): {
    happ: WebHappDefinition;
    UiTemplates: string[];
  } {
    return {
      UiTemplates,
      happ: {
        happ: newHappDef(),
        uiTemplate: 'svelte',
      },
    };
  },
  methods: {
    requestScaffold() {
      this.$emit('scaffoldApp', this.happ);
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