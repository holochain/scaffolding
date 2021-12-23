<template>
  <mwc-card style="display: flex; flex-direction: column">
    <div style="display: flex; flex-direction: row">
      <div style="display: flex; flex-direction: column; flex: 1; margin: 16px; margin-bottom: 0">
        <span style="font-size: 18px">Dna: {{ dna.name }}</span>
        <mwc-textfield
          label="Dna Name"
          style="width: 424px; margin-top: 16px"
          required
          autoValidate
          :ref="`dna-name-${dnaIndex}`"
          outlined
          helper="Has to be unique within the hApp, and snake_case"
          validationMessage="Must not be empty"
          @focus="dnaNameValidity(dnaIndex, $event.target)"
          @input="setDnaName(dnaIndex, $event.target)"
        ></mwc-textfield>
      </div>

      <mwc-icon-button :disabled="happ.dnas.length < 2" icon="delete" @click="deleteDna()"></mwc-icon-button>
    </div>

    <span style="margin-left: 16px; margin-bottom: 8px; font-size: 18px">Zomes</span>

    <div style="display: flex; flex-direction: row; flex-wrap: wrap; background-color: lightgrey; align-items: center">
      <mwc-card
        v-for="(zome, zomeIndex) of dna.zomes"
        :key="zomeIndex"
        @click="selectZome(dnaIndex, zomeIndex)"
        style="width: 150px; height: 50px; margin: 16px; cursor: pointer"
      >
        <div
          :style="{
            'background-color': this.selectedZomeIndex === zomeIndex ? 'rgba(0,0,0,0.1)' : 'white',
            display: 'flex',
            flex: '1',
          }"
        >
          <span style="margin: 16px">{{ zome.name }}</span>
        </div>
      </mwc-card>
      <mwc-icon-button icon="add" @click="addZome(dnaIndex)"></mwc-icon-button>
    </div>
  </mwc-card>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import { newZome } from '../utils';
import { TextField } from '@material/mwc-textfield';
import { isSnakeCase } from '@holochain/rad-generators';

export default defineComponent({
  name: 'DefineDna',

  props: {
    happ: { type: Object as PropType<HappDefinition>, required: true },
    dnaIndex: { type: Number, required: true },
    selectedZomeIndex: { type: Number, required: false },
  },
  computed: {
    dna() {
      return this.happ.dnas[this.dnaIndex];
    },
  },
  methods: {
    dnaNameValidity(dnaIndex: number, textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }
        if (!isSnakeCase(newValue)) {
          textfield.setCustomValidity('The DNA name must be snake_case');
          return {
            valid: false,
          };
        }

        if (this.happ.dnas.find((dna, index) => index !== dnaIndex && dna.name === newValue)) {
          textfield.setCustomValidity('The DNA name has to be unique');
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
    newZomeName(dnaIndex: number) {
      const dna = this.happ.dnas[dnaIndex];
      for (let i = 0; i < dna.zomes.length + 1; i++) {
        const name = `zome_${i}`;
        if (!dna.zomes.find(zome => zome.name === name)) {
          return name;
        }
      }
      return `zome_${dna.zomes.length + 1}`;
    },
    deleteDna() {
      this.happ.dnas.splice(this.dnaIndex, 1);
      this.$emit('dna-deleted', this.happ);
    },
    addZome(dnaIndex: number) {
      const zomes = this.happ.dnas[dnaIndex].zomes;
      zomes.push(newZome(this.newZomeName(dnaIndex)));

      this.$forceUpdate();
      this.emitChanged();
    },
    setDnaName(dnaIndex: number, textfield: TextField) {
      if (textfield.validity.valid) {
        this.happ.dnas[dnaIndex].name = textfield.value;
      }
      this.emitChanged();
    },
    emitChanged() {
      this.$forceUpdate();

      this.$emit('dna-changed', this.happ);
    },
    selectZome(dnaIndex: number, zomeIndex: number) {
      this.$emit('zome-selected', {
        dnaIndex,
        zomeIndex,
      });
    },
  },
});
</script>