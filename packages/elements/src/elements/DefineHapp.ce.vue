<template>
  <div style="display: flex; flex-direction: column; flex: 1">
    <mwc-card style="display: flex">
      <div style="display: flex; flex-direction: column; margin: 16px">
        <span style="font-size: 18px">hApp: {{ happ.name }}</span>
        <div style="display: flex; flex-direction: row; margin-top: 16px">
          <div style="flex: 1">
            <mwc-textfield
              label="hApp Name"
              style="width: 424px"
              required
              autoValidate
              outlined
              validationMessage="Must not be empty"
              @input="setHappName($event.target)"
            ></mwc-textfield>
          </div>
          <slot></slot>
        </div>
      </div>
    </mwc-card>

    <div style="display: flex; flex-direction: row; align-items: center; margin-top: 16px">
      <span style="flex: 1; font-size: 18px">Dnas</span>
      <mwc-button icon="add" label="Add Dna" @click="addDna()"></mwc-button>
    </div>
    <DefineDna
      v-for="(dna, dnaIndex) of happ.dnas"
      :key="key + dnaIndex"
      style="margin-top: 16px"
      :happ="happ"
      :dnaIndex="dnaIndex"
      :selectedZomeIndex="dnaIndex === selectedDnaIndex ? selectedZomeIndex : undefined"
      @zome-selected="$emit('zome-selected', $event)"
      @dna-changed="emitChanged()"
      @dna-deleted="onDnaDeleted(dnaIndex)"
    ></DefineDna>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import { newDna } from '../utils';
import DefineDna from './DefineDna.ce.vue';
import { TextField } from '@material/mwc-textfield';

export default defineComponent({
  name: 'DefineHapp',
  components: { DefineDna },

  props: {
    happ: {
      type: Object as PropType<HappDefinition>,
      required: true,
    },
    selectedDnaIndex: {
      type: Number,
      required: false,
    },
    selectedZomeIndex: {
      type: Number,
      required: false,
    },
  },
  data(): { key: number } {
    return {
      key: 0,
    };
  },
  methods: {
    newDnaName() {
      for (let i = 0; i < this.happ.dnas.length + 1; i++) {
        const name = `dna_${i}`;
        if (!this.happ.dnas.find(dna => dna.name === name)) {
          return name;
        }
      }
      return `dna_${this.happ.dnas.length + 1}`;
    },
    setHappName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.happ.name = textfield.value;
      }
      this.emitChanged();
    },
    addDna() {
      const name = this.newDnaName();
      this.happ.dnas.push(newDna(name));

      this.key += 100;
      this.emitChanged();
    },
    onDnaDeleted(dnaIndex: number) {
      this.key += 100;
      this.$emit('dna-deleted', dnaIndex);
    },
    emitChanged() {
      this.$forceUpdate();

      this.$emit('happ-changed', this.happ);
    },
  },
});
</script>