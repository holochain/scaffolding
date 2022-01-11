<template>
  <mwc-card style="display: flex; flex: 1; flex-direction: row">
    <DefineHapp :happ="happ" @happ-changed="emitChanged()">
      <mwc-select
        v-if="uitemplates"
        outlined
        :fixedMenuPosition="true"
        label="UI Template"
        value="svelte"
        ref="uiTemplateSelect"
        @change="selectedUiTemplate = $event.target.value"
      >
        <mwc-list-item v-for="(ui, index) of uitemplates.split(',')" :key="index" :value="ui">{{ ui }}</mwc-list-item>
      </mwc-select>
    </DefineHapp>
  </mwc-card>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import type { Select } from '@material/mwc-select';
import DefineHapp from './DefineHapp.ce.vue';
import { newHappDef } from '../utils';

export default defineComponent({
  name: 'WebHappDefinitionBuilder',

  components: {
    DefineHapp,
  },
  props: {
    happ: {
      type: Object as PropType<HappDefinition>,
      required: false,
      default: newHappDef(),
    },
    uitemplates: {
      type: String,
      required: false,
    },
  },
  data(): { selectedUiTemplate: string } {
    return {
      selectedUiTemplate: this.uitemplates ? this.uitemplates[0] : '',
    };
  },
  methods: {
    emitChanged() {
      this.$emit('happ-changed', { happ: this.happ, uiTemplate: this.selectedUiTemplate });
    },
  },
});
</script>