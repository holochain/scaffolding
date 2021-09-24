<template>
  <div class="column">
    <h1>Scaffold New App</h1>
    <ui5-card style="width: auto">
      <div class="column" style="margin: 16px">
        <span class="secondary-title">App Information</span>
        <mwc-textfield
          label="hApp Name"
          class="text-input"
          required
          autoValidate
          outlined
          :value="happ.name"
          @input="happ.name = $event.target.value"
          style="margin-top: 8px"
        ></mwc-textfield>
      </div>
    </ui5-card>

    <div style="margin-top: 16px; margin-bottom: 12px" class="row center">
      <span class="secondary-title" style="flex: 1">Dna Slots</span>
      <mwc-button
        icon="add"
        label="Add dna"
        @click="
          happ.dnas.push({
            name: 'my-dna',
            zomes: [
              {
                name: 'my-zome',
              },
            ],
          })
        "
      ></mwc-button>
    </div>
    <ui5-card style="width: auto; margin-bottom: 16px" v-for="(dna, index) of happ.dnas" :key="dna.name">
      <div class="column">
        <div class="column" style="margin: 16px">
          <div class="row">
            <span style="flex: 1; font-size: 18px">Dna Slot #{{ index + 1 }}</span>
            <mwc-icon-button
              :disabled="happ.dnas.length < 2"
              @click="happ.dnas.splice(index, 1)"
              icon="delete"
            ></mwc-icon-button>
          </div>

          <mwc-textfield
            label="DNA Name"
            required
            outlined
            autoValidate
            @input="dna.name = $event.target.value"
            class="text-input"
            :value="dna.name"
          ></mwc-textfield>
        </div>
        <div class="row center" style="margin: 4px 16px">
          <span style="flex: 1; font-size: 18px">Zomes</span>
          <mwc-button icon="add" label="Add zome" @click="dna.zomes.push({ name: 'new-zome' })"></mwc-button>
        </div>
        <span style="width: 100%; height: 1px; background-color: lightgrey"></span>
        <div class="column">
          <div class="row">
            <mwc-list style="width: 200px" activatable>
              <mwc-list-item
                v-for="(zome, index) of dna.zomes"
                :key="zome.name"
                :activated="selectedZomes[dna.name] === index"
                @click="selectedZomes[dna.name] = index"
              >
                {{ zome.name }}
              </mwc-list-item>
            </mwc-list>

            <div class="row center" style="flex: 1; align-self: start">
              <mwc-textfield
                label="Zome Name"
                class="text-input"
                :value="dna.zomes[selectedZomes[dna.name]].name"
                @input="dna.zomes[selectedZomes[dna.name]].name = $event.target.value"
                required
                outlined
                autoValidate
                style="flex: 1; margin: 8px"
              ></mwc-textfield>

              <mwc-icon-button :disabled="dna.zomes.length < 2" @click="deleteSelectedZome(index)" icon="delete">
              </mwc-icon-button>
            </div>
          </div>
        </div>
      </div>
    </ui5-card>
  </div>

  <mwc-fab
    @click="$emit('scaffoldApp', happ)"
    style="position: fixed; bottom: 16px; right: 16px"
    label="Scaffold app"
    extended
    icon="system_update_alt"
  ></mwc-fab>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { socket } from '../socket';
import { ClientEventType } from '@holochain/scaffolding-types';
import { generateWebHapp, HappDefinition } from '@holochain/scaffolding-generators';

export default defineComponent({
  name: 'AppDefinitionBuilder',

  data(): { happ: HappDefinition; selectedZomes: { [dna: string]: number } } {
    return {
      selectedZomes: {
        'my-dna': 0,
      },
      happ: {
        name: 'my-app',
        dnas: [
          {
            name: 'my-dna',
            zomes: [
              {
                name: 'my-zome',
              },
            ],
          },
        ],
      },
    };
  },
  methods: {
    deleteSelectedZome(dnaIndex: number) {
      const dna = this.happ.dnas[dnaIndex];
      const selectedZome = this.selectedZomes[dna.name];

      dna.zomes.splice(selectedZome, 1);
      if (this.selectedZomes[dna.name] !== 0) this.selectedZomes[dna.name]--;
    },
  },
  emits: ['scaffoldApp'],
});
</script>
<style scoped>
.text-input {
  width: 424px;
}

.secondary-title {
  font-size: 18px;
}
</style>