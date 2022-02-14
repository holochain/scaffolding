<template>
  <div style="display: flex; flex-direction: row; flex: 1">
    <div style="display: flex; flex-direction: column; box-shadow: rgb(0 0 0 / 23%) 1px 0px 6px">
      <div style="display: flex; flex-direction: column; margin: 16px">
        <span style="font-size: 20px">hApp: {{ happ.name }}</span>
        <mwc-textfield
          label="hApp Name"
          style="width: 424px; margin-top: 24px; margin-bottom: 16px"
          required
          autoValidate
          ref="happ-name"
          outlined
          validationMessage="Must not be empty"
          @input="setHappName($event.target)"
        ></mwc-textfield>
        <slot></slot>
      </div>

      <span style="font-size: 18px; margin: 16px">DNAs</span>
      <span style="width: 100%; height: 1px; background-color: lightgrey"></span>
      <div style="position: relative; display: flex; flex: 1">
        <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0">
          <div style="max-height: 100%; overflow-y: auto">
            <mwc-list activatable>
              <mwc-list-item
                graphic="icon"
                v-for="(dna, dnaIndex) of happ.dnas"
                :key="dnaIndex"
                :activated="selectedDnaIndex === dnaIndex"
                @click="selectedDnaIndex = dnaIndex"
              >
                <mwc-icon slot="graphic">hive</mwc-icon>
                {{ dna.name }}
              </mwc-list-item>
            </mwc-list>
          </div>
        </div>
      </div>
      <mwc-button icon="add" label="Add Dna" @click="addDna()"></mwc-button>
    </div>

    <span style="height: 100%; width: 1px; background-color: lightgrey"></span>

    <DefineDna
      v-if="selectedDna"
      :dna="selectedDna"
      :key="selectedDnaIndex"
      :otherDnasNames="otherDnasNames"
      @dna-changed="emitChanged()"
    >
      <mwc-button
        label="Remove DNA"
        :disabled="happ.dnas.length < 2"
        icon="delete"
        @click="deleteDna()"
        style="margin: 8px; --mdc-theme-primary: black"
      ></mwc-button>
    </DefineDna>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain-scaffolding/definitions';
import { newDnaDef, newHappDef } from '../utils';
import DefineDna from './DefineDna.ce.vue';
import { TextField } from '@material/mwc-textfield';

export default defineComponent({
  name: 'DefineHapp',
  components: { DefineDna },

  props: {
    happ: {
      type: Object as PropType<HappDefinition>,
      required: false,
      default: newHappDef(),
    },
  },
  data(): { dnaCount: number; selectedDnaIndex: number } {
    return {
      dnaCount: 1,
      selectedDnaIndex: 0,
    };
  },
  mounted() {
    setTimeout(() => {
      const field = this.$refs['happ-name'] as TextField;
      field.value = this.happ.name;
    }, 1);
  },
  computed: {
    selectedDna() {
      if (this.selectedDnaIndex < 0) return undefined;
      return this.happ.dnas[this.selectedDnaIndex];
    },
    otherDnasNames() {
      return this.happ.dnas.filter((_, index) => index !== this.selectedDnaIndex).map(dna => dna.name);
    },
  },
  methods: {
    setHappName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.happ.name = textfield.value;
      }
      this.emitChanged();
    },
    addDna() {
      const name = `dna_${this.dnaCount++}`;

      this.happ.dnas.push(newDnaDef(name));
      this.selectedDnaIndex = this.happ.dnas.length - 1;

      this.emitChanged();
    },
    deleteDna() {
      this.happ.dnas.splice(this.selectedDnaIndex, 1);
      this.selectedDnaIndex--;
      if (this.selectedDnaIndex < 0) this.selectedDnaIndex = 0;
      this.emitChanged();
    },
    emitChanged() {
      this.$forceUpdate();

      this.$emit('happ-changed', this.happ);
    },
  },
});
</script>