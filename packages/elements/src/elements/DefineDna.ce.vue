<template>
  <div style="display: flex; flex-direction: column; flex: 1">
    <div style="display: flex; flex-direction: column; box-shadow: 0 5px 6px -4px rgb(0 0 0 / 23%)">
      <div style="display: flex; flex-direction: row">
        <div style="display: flex; flex-direction: column; flex: 1; margin: 16px; margin-bottom: 0">
          <span style="font-size: 20px">DNA: {{ dna.name }}</span>
          <mwc-textfield
            label="DNA Name"
            style="width: 424px; margin-top: 24px"
            required
            autoValidate
            outlined
            ref="dna-name"
            helper="Has to be unique within the hApp, and snake_case"
            validationMessage="Must not be empty"
            @focus="dnaNameValidity($event.target)"
            @input="setDnaName($event.target)"
          ></mwc-textfield>
        </div>

        <slot></slot>
      </div>

      <div style="display: flex; flex-direction: row; align-items: center">
        <mwc-tab-bar :activeIndex="selectedIntegrityZomeIndex">
          <mwc-tab
            v-for="(integrityZome, integrityZomeIndex) of dna.integrity_zomes"
            :key="integrityZome.name"
            :label="integrityZome.name"
            @click="selectedIntegrityZomeIndex = integrityZomeIndex"
          ></mwc-tab>
        </mwc-tab-bar>

        <span style="flex: 1"></span>

        <mwc-button icon="add" @click="addZome()" label="Add Zome" style="margin-right: 8px"></mwc-button>
      </div>
      <span style="width: 100%; height: 1px; background-color: lightgrey"></span>
    </div>

    <DefineIntegrityZome
      v-if="selectedIntegrityZome"
      :key="selectedIntegrityZomeIndex"
      :integrityZome="selectedIntegrityZome"
      :otherZomesNames="otherZomesNames"
      @zome-changed="emitChanged()"
      style="margin-top: 8px"
    >
      <mwc-button
        label="Remove Zome"
        :disabled="dna.integrity_zomes.length < 2"
        icon="delete"
        @click="deleteZome()"
        style="margin: 8px; --mdc-theme-primary: black"
      ></mwc-button>
    </DefineIntegrityZome>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import {
  DnaDefinition,
  newDnaDef,
  newIntegrityZomeDef,
  newCoordinatorZomeDef,
} from '@holochain-scaffolding/definitions';
import { TextField } from '@material/mwc-textfield';
import { isSnakeCase } from '@holochain-scaffolding/generators';
import DefineIntegrityZome from './DefineIntegrityZome.ce.vue';

export default defineComponent({
  name: 'DefineDna',

  components: {
    DefineIntegrityZome,
  },

  props: {
    dna: { type: Object as PropType<DnaDefinition>, required: false, default: newDnaDef() },
    otherDnaNames: { type: Array, required: false, default: [] },
  },
  data(): { integrityZomeCount: number; selectedIntegrityZomeIndex: number } {
    return {
      integrityZomeCount: 1,
      selectedIntegrityZomeIndex: 0,
    };
  },
  computed: {
    otherZomesNames() {
      return [...this.dna.integrity_zomes, ...this.dna.coordinator_zomes]
        .filter((_, index) => index !== this.selectedIntegrityZomeIndex)
        .map(integrityZome => integrityZome.name);
    },
    selectedIntegrityZome() {
      if (this.selectedIntegrityZomeIndex < 0) return undefined;
      return this.dna.integrity_zomes[this.selectedIntegrityZomeIndex];
    },
  },
  watch: {
    dna() {
      this.updateDnaName();
      this.selectedIntegrityZomeIndex = 0;
    },
  },
  mounted() {
    this.updateDnaName();
  },
  methods: {
    updateDnaName() {
      setTimeout(() => {
        const field = this.$refs['dna-name'] as TextField;
        field.value = this.dna.name;
      }, 1);
    },
    dnaNameValidity(textfield: TextField) {
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

        if (this.otherDnaNames.includes(newValue)) {
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
    deleteZome() {
      this.dna.integrity_zomes.splice(this.selectedIntegrityZomeIndex, 1);
      this.selectedIntegrityZomeIndex--;
      if (this.selectedIntegrityZomeIndex < 0) this.selectedIntegrityZomeIndex = 0;
      this.emitChanged();
    },
    addZome() {
      const name = `zome_${this.integrityZomeCount++}`;
      const zomeBundleDef = newIntegrityZomeDef(`${name}`);
      this.dna.integrity_zomes.push(zomeBundleDef);
      this.selectedIntegrityZomeIndex = this.dna.integrity_zomes.length - 1;

      this.emitChanged();
    },
    setDnaName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.dna.name = textfield.value;
      }
      this.emitChanged();
    },
    emitChanged() {
      this.$forceUpdate();

      this.$emit('dna-changed', this.dna);
    },
  },
});
</script>
