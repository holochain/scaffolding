<template>
  <mwc-card style="display: flex; flex: 1; flex-direction: row">
    <DefineHapp :happ="happ" @happ-changed="emitChanged()">
      <mwc-select
        v-if="uitemplates"
        outlined
        :fixedMenuPosition="true"
        label="UI Template"
        ref="uiTemplateSelect"
        @selected="onSelect($event.detail.index)"
      >
        <mwc-list-item v-for="(ui, index) of uitemplates.split(',')" :key="index" :value="ui">{{ ui }}</mwc-list-item>
      </mwc-select>
    </DefineHapp>
  </mwc-card>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition, newHappDef } from '@holochain-scaffolding/definitions';
import DefineHapp from './DefineHapp.ce.vue';

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
      required: true,
    },
  },
  data(): { selectedUiTemplate: string } {
    return {
      selectedUiTemplate: this.uitemplates ? this.uitemplates[0] : '',
    };
  },
  mounted() {
    (this.$refs['uiTemplateSelect'] as any).value = this.selectedUiTemplate;
  },
  methods: {
    onSelect(index: number) {
      this.selectedUiTemplate = this.uitemplates.split(',')[index];
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('happ-changed', { happ: this.happ, uiTemplate: this.selectedUiTemplate });
    },
  },
});
</script>