<template>
  <mwc-card style="flex: 1; display: flex">
    <div style="display: flex; flex: 1; flex-direction: column">
      <sl-breadcrumb style="margin: 16px">
        <sl-breadcrumb-item @click="selectHapp()">hApp</sl-breadcrumb-item>
        <sl-breadcrumb-item v-if="selectedDna" @click="selectDna()">Dna: {{ selectedDna.name }}</sl-breadcrumb-item>
        <sl-breadcrumb-item v-if="selectedZome" @click="selectZome()">Zome: {{ selectedZome.name }}</sl-breadcrumb-item>
        <sl-breadcrumb-item v-if="selectedEntryDef">Entry Def: {{ selectedEntryDef.name }}</sl-breadcrumb-item>
      </sl-breadcrumb>

      <span style="height: 1px; width: 100%; background-color: lightgrey"></span>

      <div style="margin: 16px">
        <div v-if="selectedEntryDef">
          <DefineEntry
            :entryDef="selectedEntryDef"
            :otherEntryDefsNames="otherEntryDefsNames()"
            @entry-def-changed="onChanged()"
          ></DefineEntry>
        </div>
        <div v-else-if="selectedZome">
          <DefineZome
            :zome="selectedZome"
            :otherZomesNames="otherZomesNames()"
            @zome-changed="onChanged()"
            @entry-def-selected="selectedEntryDefIndex = $event"
            @entry-def-added="selectedEntryDefIndex = $event"
            @entry-def-deleted="selectedEntryDefIndex = -1"
          ></DefineZome>
        </div>
        <div v-else-if="selectedDna">
          <DefineDna
            :dna="selectedDna"
            :otherDnasNames="otherDnasNames()"
            @dna-changed="onChanged()"
            @zome-selected="selectedZomeIndex = $event"
            @zome-added="selectedZomeIndex = $event"
            @zome-deleted="selectedZomeIndex = -1"
          ></DefineDna>
        </div>

        <div v-else>
          <DefineHapp
            :happ="happ"
            @happ-changed="onChanged()"
            @dna-selected="selectedDnaIndex = $event"
            @dna-added="selectedDnaIndex = $event"
            @dna-deleted="selectedDnaIndex = -1"
          >
            <mwc-select
              v-if="uiTemplates"
              outlined
              slot="additionalProperty"
              label="UI Template"
              value="svelte"
              ref="uiTemplateSelect"
              style="right: 16px; position: absolute"
            >
              <mwc-list-item v-for="(ui, index) of uiTemplates" :key="index" :value="ui">{{ ui }}</mwc-list-item>
            </mwc-select>
          </DefineHapp>
        </div>
      </div>
    </div>
  </mwc-card>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { DnaDefinition, HappDefinition } from '@holochain/rad-definitions';
import DefineHapp from './DefineHapp.ce.vue';
import DefineDna from './DefineDna.ce.vue';
import DefineZome from './DefineZome.ce.vue';
import DefineEntry from './DefineEntry.ce.vue';
import { newDna } from '../utils';

export default defineComponent({
  name: 'HappDefinitionBuilder',

  components: {
    DefineHapp,
    DefineDna,
    DefineZome,
    DefineEntry,
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
    uiTemplates: {
      type: Object as PropType<Array<string>>,
      required: false,
    },
  },
  data(): {
    selectedDnaIndex: number;
    selectedZomeIndex: number;
    selectedEntryDefIndex: number;
    dnaCount: number;
  } {
    return {
      selectedDnaIndex: -1,
      selectedZomeIndex: -1,
      selectedEntryDefIndex: -1,
      dnaCount: 1,
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
    selectedEntryDef() {
      if (!this.selectedZome || this.selectedEntryDefIndex === -1) return undefined;
      return this.selectedZome.entry_defs[this.selectedEntryDefIndex];
    },
  },
  methods: {
    otherDnasNames() {
      return this.happ.dnas.filter((_, index) => index !== this.selectedDnaIndex).map(dna => dna.name);
    },
    otherZomesNames() {
      return this.selectedDna?.zomes.filter((_, index) => index !== this.selectedZomeIndex).map(zome => zome.name);
    },
    otherEntryDefsNames() {
      return this.selectedZome?.entry_defs
        .filter((_, index) => index !== this.selectedEntryDefIndex)
        .map(entryDef => entryDef.name);
    },
    onChanged() {
      this.$forceUpdate();
      this.emitChanged();
    },
    selectZome() {
      this.selectedEntryDefIndex = -1;
    },
    selectDna() {
      this.selectedEntryDefIndex = -1;
      this.selectedZomeIndex = -1;
    },
    selectHapp() {
      this.selectedEntryDefIndex = -1;
      this.selectedZomeIndex = -1;
      this.selectedDnaIndex = -1;
    },
    emitChanged() {
      this.$emit('happ-changed', this.happ);
    },
  },
});
</script>