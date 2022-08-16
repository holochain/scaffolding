<template>
  <mwc-card style="display: flex; flex: 1; flex-direction: row">
    <DefineHapp :happ="happ" @happ-changed="(e) => this.handleHappChange(e)">
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
import { DnaDefinition, HappDefinition, newHappDef } from '@holochain-scaffolding/definitions';
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
      selectedUiTemplate: this.uitemplates ? this.uitemplates.split(',')[0] : '',
    };
  },
  mounted() {
    (this.$refs['uiTemplateSelect'] as any).value = this.selectedUiTemplate;
  },
  methods: {
    handleHappChange(happ: HappDefinition) {
      this.happ.name = happ.name;
      this.happ.dnas = happ.dnas;
      this.emitChanged();
    },
    onSelect(index: number) {
      this.selectedUiTemplate = this.uitemplates.split(',')[index];
      this.emitChanged();
    },
    emitChanged() {

      const outDnas = this.happ.dnas.map((dna) => {
        const outDna: DnaDefinition = {
          name: dna.name,
          coordinator_zomes: dna.integrity_zomes.map(iz => ({
            name: iz.name,
            dependencies: [`${iz.name}_integrity`],
          })),
          integrity_zomes: dna.integrity_zomes.map(iz => ({
            name: `${iz.name}_integrity`,
            entry_defs: iz.entry_defs,
          })),
        }
        return outDna;
      })

      const outHapp: HappDefinition = {
        name: this.happ.name,
        dnas: outDnas,
      };

      this.$emit('happ-changed', { happ: outHapp, uiTemplate: this.selectedUiTemplate });
    },
  },
});
</script>