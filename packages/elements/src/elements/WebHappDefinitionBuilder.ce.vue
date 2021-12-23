<template>
  <div style="display: flex; flex: 1; flex-direction: row">
    <div style="position: relative; display: flex; flex: 1">
      <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0">
        <div style="max-height: 100%; overflow-y: auto">
          <DefineHapp
            :key="dnaRender"
            :happ="happ"
            :selectedDnaIndex="selectedDnaIndex"
            :selectedZomeIndex="selectedZomeIndex"
            @happ-changed="onChanged()"
            @zome-selected="onZomeSelected($event)"
            @dna-deleted="onDnaDeleted($event)"
          >
            <mwc-select
              v-if="uitemplates"
              outlined
              :fixedMenuPosition="true"
              label="UI Template"
              value="svelte"
              ref="uiTemplateSelect"
              @change="emitChanged()"
            >
              <mwc-list-item v-for="(ui, index) of uitemplates.split(',')" :key="index" :value="ui">{{
                ui
              }}</mwc-list-item>
            </mwc-select>
          </DefineHapp>
        </div>
      </div>
    </div>

    <mwc-card style="flex: 1; display: flex; margin-left: 16px">
      <DefineZome
        v-if="selectedZome"
        :key="zomeRender"
        :happ="happ"
        :dnaIndex="selectedDnaIndex"
        :zomeIndex="selectedZomeIndex"
        @zome-changed="dnaRender++"
        @zome-deleted="onZomeDeleted()"
      ></DefineZome>
      <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
        <span style="opacity: 0.6">Select a zome to define it</span>
      </div>
    </mwc-card>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import type { Select } from '@material/mwc-select';
import DefineHapp from './DefineHapp.ce.vue';
import DefineZome from './DefineZome.ce.vue';
import { newDna } from '../utils';

export default defineComponent({
  name: 'WebHappDefinitionBuilder',

  components: {
    DefineHapp,
    DefineZome,
  },
  props: {
    happ: {
      type: Object as PropType<HappDefinition>,
      required: false,
      default: {
        name: 'my-app',
        dnas: [newDna()],
      },
    },
    uitemplates: {
      type: String,
      required: false,
    },
  },
  data(): {
    selectedDnaIndex: number;
    selectedZomeIndex: number;
    dnaRender: number;
    zomeRender: number;
  } {
    return {
      selectedDnaIndex: -1,
      selectedZomeIndex: -1,
      dnaRender: 0,
      zomeRender: 0,
    };
  },
  computed: {
    selectedDna() {
      if (this.selectedDnaIndex === -1) return undefined;
      return this.happ.dnas[this.selectedDnaIndex];
    },
    selectedZome() {
      if (!this.selectedDna || this.selectedZomeIndex === -1) return undefined;
      return this.selectedDna.zomes[this.selectedZomeIndex];
    },
  },
  methods: {
    onChanged() {
      this.zomeRender++;
      this.$forceUpdate();
      this.emitChanged();
    },
    onZomeSelected({ dnaIndex, zomeIndex }: { dnaIndex: number; zomeIndex: number }) {
      this.selectedDnaIndex = dnaIndex;
      this.selectedZomeIndex = zomeIndex;
    },
    emitChanged() {
      const uiTemplateSelect = this.$refs['uiTemplateSelect'] as Select;
      this.$emit('happ-changed', { happ: this.happ, uiTemplate: uiTemplateSelect.value });
    },
    onDnaDeleted(dnaIndex: number) {
      if (this.selectedDnaIndex === dnaIndex) {
        this.selectedDnaIndex = -1;
        this.selectedZomeIndex = -1;
      }
      if (this.selectedDnaIndex > dnaIndex) this.selectedDnaIndex -= 1;
      this.zomeRender++;
    },
    onZomeDeleted() {
      this.dnaRender++;
      this.selectedDnaIndex = -1;
      this.selectedZomeIndex = -1;
    },
  },
});
</script>