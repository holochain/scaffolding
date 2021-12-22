<template>
  <div style="display: flex; flex-direction: column; flex: 1">
    <div style="display: flex; flex-direction: row">
      <div style="display: flex; flex-direction: column; flex: 1">
        <span style="font-size: 18px">Dna: {{ selectedDna.name }} > Zome: {{ zome.name }}</span>
        <mwc-textfield
          label="Zome Name"
          style="width: 424px; margin-top: 16px"
          required
          autoValidate
          ref="zome-name"
          outlined
          validationMessage="Must not be empty"
          @focus="zomeValidity($event.target)"
          @input="setZomeName($event.target)"
        ></mwc-textfield>
      </div>

      <mwc-icon-button :disabled="selectedDna.zomes.length < 2" icon="delete" @click="deleteZome()"></mwc-icon-button>
    </div>

    <div style="display: flex; flex-direction: row">
      <span style="flex: 1; font-size: 16px">Entry Defs</span>
    </div>
    <div style="display: flex; flex-direction: row; flex: 1">
      <div style="display: flex; flex-direction: column">
        <mwc-list activatable>
          <div
            style="display: flex; flex-direction: row; flex: 1"
            v-for="(entryDef, entryDefIndex) of zome.entry_defs"
            :key="entryDefIndex"
          >
            <mwc-list-item
              :activated="selectedEntryDefIndex === entryDefIndex"
              @click="selectedEntryDefIndex = entryDefIndex"
              style="flex: 1"
            >
              {{ entryDef.name }}
            </mwc-list-item>
          </div>
        </mwc-list>
        <mwc-button icon="add" label="Add Entry Def" @click="addEntryDef()"></mwc-button>
      </div>

      <DefineEntry
        v-if="selectedEntryDef"
        :happ="happ"
        :dnaIndex="dnaIndex"
        :zomeIndex="zomeIndex"
        :entryDefIndex="selectedEntryDefIndex"
        @entry-def-deleted="onEntryDefDeleted(selectedEntryDefIndex)"
      ></DefineEntry>
      <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
        <span style="opacity: 0.6">Select an entry def </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { isSnakeCase } from '@holochain/rad-generators';
import { HappDefinition, ZomeDefinition } from '@holochain/rad-definitions';
import { newEntryDef } from '../utils';
import DefineEntry from './DefineEntry.ce.vue';

export default defineComponent({
  name: 'DefineZome',

  components: {
    DefineEntry,
  },

  props: {
    happ: { type: Object as PropType<HappDefinition>, required: true },
    dnaIndex: { type: Number, required: true },
    zomeIndex: { type: Number, required: true },
  },
  data(): { entryDefCount: number; selectedEntryDefIndex: number } {
    return {
      entryDefCount: 1,
      selectedEntryDefIndex: -1,
    };
  },
  mounted() {
    const field = this.$refs['zome-name'] as TextField;
    field.value = this.zome.name;
  },
  watch: {
    dnaIndex: function () {
      const field = this.$refs['zome-name'] as TextField;
      field.value = this.zome.name;
    },
    zomeIndex: function () {
      const field = this.$refs['zome-name'] as TextField;
      field.value = this.zome.name;
    },
  },
  computed: {
    selectedEntryDef() {
      if (this.selectedEntryDefIndex === -1) return undefined;
      else return this.zome.entry_defs[this.selectedEntryDefIndex];
    },
    otherZomesNames() {
      return this.selectedDna?.zomes.filter((_, index) => index !== this.zomeIndex).map(zome => zome.name);
    },
    selectedDna() {
      return this.happ.dnas[this.dnaIndex];
    },
    zome() {
      return this.selectedDna.zomes[this.zomeIndex];
    },
  },
  methods: {
    addEntryDef() {
      const name = `entry_def_${this.entryDefCount++}`;
      this.zome.entry_defs.push(newEntryDef(name));
      this.emitChanged();
    },
    onEntryDefDeleted(entryDefIndex: number) {
      this.selectedEntryDefIndex = -1;
      this.emitChanged();
    },
    zomeValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }

        if (!isSnakeCase(newValue)) {
          textfield.setCustomValidity('The zome name must be snake_case');
          return {
            valid: false,
          };
        }

        if (this.otherZomesNames.find(name => name === newValue)) {
          textfield.setCustomValidity('The zome name has to be unique');
          return {
            valid: false,
          };
        }
        textfield.setCustomValidity('');
        return {
          valid: true,
        };
      };
    },
    setZomeName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.zome.name = textfield.value;
      }
      this.emitChanged();
    },
    deleteZome() {
      this.selectedDna.zomes.splice(this.zomeIndex, 1);
      this.$emit('zome-deleted');
    },
    emitChanged() {
      this.$forceUpdate();
      this.$emit('zome-changed', this.zome);
    },
  },
});
</script>