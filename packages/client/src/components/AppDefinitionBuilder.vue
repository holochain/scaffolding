<template>
  <div class="column" style="position: relative">
    <h1>Scaffold New App</h1>

    <ui5-card style="width: auto">
      <div class="column" style="margin: 16px">
        <span class="tertiary-title">App Information</span>
        <div class="row" style="margin-top: 16px">
          <mwc-textfield
            label="hApp Name"
            class="text-input"
            required
            autoValidate
            outlined
            helper="The name of your app"
            @input="happ.name = $event.target.value"
            ref="happName"
          ></mwc-textfield>
          <span style="flex: 1"></span>

          <mwc-select
            outlined
            label="UI Template"
            ref="uiTemplateSelect"
            @selected="selectUi($event.detail.index)"
            style="right: 16px; position: absolute"
          >
            <mwc-list-item v-for="(ui, index) of UiTemplates" :key="index" :value="`${index}`" :selected="index == 0">{{
              UiTemplates[index]
            }}</mwc-list-item>
          </mwc-select>
        </div>
      </div>
    </ui5-card>

    <div style="margin-top: 16px; margin-bottom: 12px" class="row center">
      <span class="secondary-title" style="flex: 1">DNA Slots</span>
      <mwc-button icon="add" label="Add dna" @click="addDna()"></mwc-button>
    </div>
    <ui5-card style="width: auto; margin-bottom: 16px" v-for="(dna, dnaIndex) of happ.dnas" :key="dnaIndex">
      <div class="column">
        <div class="row">
          <div class="column" style="margin: 16px; margin-bottom: 4px; flex: 1">
            <span style="flex: 1" class="tertiary-title">DNA: {{ dna.name }}</span>

            <mwc-textfield
              label="DNA Name"
              outlined
              required
              :name="`dna-${dnaIndex}`"
              :ref="`dna-${dnaIndex}`"
              style="margin-top: 16px"
              @focus="dnaValidity($event.target, dna.name)"
              @input="$event.target.validity.valid && setDnaName(dnaIndex, $event.target.value)"
              class="text-input"
              helper="Has to be unique"
              autoValidate
            ></mwc-textfield>
          </div>
          <mwc-icon-button
            :disabled="happ.dnas.length < 2"
            @click="happ.dnas.splice(dnaIndex, 1)"
            icon="delete"
          ></mwc-icon-button>
        </div>
        <div class="row center" style="margin: 4px 16px">
          <span style="flex: 1; font-size: 18px">Zomes</span>
          <mwc-button
            icon="add"
            label="Add zome"
            @click="dna.zomes.push({ name: `new_zome_${zomeCount++}` })"
          ></mwc-button>
        </div>
        <span style="width: 100%; height: 1px; background-color: lightgrey"></span>
        <div class="column">
          <div class="row" style="align-items: stretch">
            <mwc-list style="width: 200px" activatable>
              <mwc-list-item
                v-for="(zome, zomeIndex) of dna.zomes"
                :key="zome.name"
                :activated="selectedZomes[dnaIndex] === zomeIndex"
                @click="selectedZomes[dnaIndex] = zomeIndex"
              >
                {{ zome.name }}
              </mwc-list-item>
            </mwc-list>

            <span style="width: 1px; background-color: lightgrey"></span>

            <div class="row" style="flex: 1; align-self: start">
              <mwc-textfield
                label="Zome Name"
                class="text-input"
                @focus="zomeValidity($event.target)"
                @input="$event.target.validity.valid && setZomeName(dnaIndex, $event.target.value)"
                required
                outlined
                helper="Has to be unique"
                autoValidate
                :name="`dna-${dnaIndex}-zome-${selectedZomes[dnaIndex]}`"
                :ref="`dna-${dnaIndex}-zome-${selectedZomes[dnaIndex]}`"
                style="margin: 8px; margin-top: 16px"
              ></mwc-textfield>
              <span style="flex: 1"></span>

              <mwc-icon-button :disabled="dna.zomes.length < 2" @click="deleteSelectedZome(dnaIndex)" icon="delete">
              </mwc-icon-button>
            </div>
          </div>
        </div>
      </div>
    </ui5-card>
  </div>

  <mwc-fab
    @click="$emit('scaffoldApp', { happ, uiTemplate })"
    style="position: fixed; bottom: 16px; right: 16px; --mdc-theme-secondary: #4720e3"
    label="Scaffold app"
    extended
    icon="system_update_alt"
  ></mwc-fab>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HappDefinition, DnaDefinition, kebabToSnakeCase } from '@holochain/scaffolding-generators';
import type { TextField } from '@material/mwc-textfield';
import type { Select } from '@material/mwc-select';
import { UiTemplates } from '../types';

export default defineComponent({
  name: 'AppDefinitionBuilder',

  data(): {
    happ: HappDefinition;
    uiTemplate: string;
    selectedZomes: Array<number>;
    dnaCount: number;
    zomeCount: number;
    UiTemplates: string[];
  } {
    return {
      UiTemplates,
      selectedZomes: [0],
      dnaCount: 1,
      uiTemplate: UiTemplates[0],
      zomeCount: 1,
      happ: {
        name: 'my-app',
        dnas: [
          {
            name: 'my-dna',
            zomes: [
              {
                name: 'my_zome',
              },
            ],
          },
        ],
      },
    };
  },
  updated() {
    this.$nextTick(() => {
      setTimeout(() => {
        this.updateValues();
        (this.$refs.uiTemplateSelect as Select).value = '0';
      });
    });
  },
  mounted() {
    this.$nextTick(() => {
      setTimeout(() => {
        this.updateValues();
      });
    });
  },
  methods: {
    selectUi(index: number) {
      this.uiTemplate = UiTemplates[index];
    },
    // Workaround for the bug inside mwc-textfield
    updateValues() {
      const refs = this.$refs as any;
      refs.happName.value = this.happ.name;

      for (let i = 0; i < this.happ.dnas.length; i++) {
        const dna = this.happ.dnas[i];

        refs[`dna-${i}`].value = dna.name;

        const selectedZomeIndex = this.selectedZomes[i];
        refs[`dna-${i}-zome-${selectedZomeIndex}`].value = dna.zomes[selectedZomeIndex].name;
      }
    },
    addDna() {
      const dnaName = `new-dna-${this.dnaCount++}`;
      this.happ.dnas.push({
        name: dnaName,
        zomes: [
          {
            name: `new_zome_${this.zomeCount++}`,
          },
        ],
      });
      this.selectedZomes[this.happ.dnas.length - 1] = 0;
    },
    deleteSelectedZome(dnaIndex: number) {
      const dna = this.happ.dnas[dnaIndex];
      const selectedZome = this.selectedZomes[dnaIndex];

      dna.zomes.splice(selectedZome, 1);
      if (this.selectedZomes[dnaIndex] !== 0) this.selectedZomes[dnaIndex]--;
    },
    dnaValidity(textfield: TextField, firstValue: string) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('The DNA name must not be empty');
          return {
            valid: false,
          };
        }

        const splitted = textfield.name.split('-');
        const dnaIndex = parseInt(splitted[1]);
        if (this.happ.dnas.find((dna, myDnaIndex) => !(dnaIndex === myDnaIndex) && dna.name === newValue)) {
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
    setDnaName(dnaIndex: number, newValue: string) {
      this.happ.dnas[dnaIndex].name = newValue;
    },
    zomeValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('The zome name must not be empty');
          return {
            valid: false,
          };
        }

        const splitted = textfield.name.split('-');
        const dnaIndex = parseInt(splitted[1]);
        const zomeIndex = parseInt(splitted[3]);
        if (
          this.happ.dnas.find((dna, myDnaIndex) =>
            dna.zomes.find(
              (zome, myZomeIndex) => !(dnaIndex === myDnaIndex && zomeIndex === myZomeIndex) && zome.name === newValue,
            ),
          )
        ) {
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
    setZomeName(dnaIndex: number, newValue: string) {
      const dna: DnaDefinition = this.happ.dnas[dnaIndex];

      dna.zomes[this.selectedZomes[dnaIndex]].name = kebabToSnakeCase(newValue);
    },
  },
  emits: ['scaffoldApp'],
});
</script>
<style scoped>
.text-input {
  width: 424px;
}
</style>