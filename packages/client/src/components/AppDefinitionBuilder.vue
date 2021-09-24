<template>
  <v-container>
    <div class="column center">
      <div class="column center">
        <mwc-textfield
          label="hApp Name"
          ref="happName"
          class="text-input"
          required
          autoValidate
          @input="happ.name = $event.target.value"
        ></mwc-textfield>

        <mwc-card v-for="dna of happ.dnas" :key="dna.name" style="width: auto">
          <div class="column">
            <mwc-textfield
              label="DNA Name"
              required
              autoValidate
              @input="dna.name = $event.target.value"
              class="text-input"
              :value="dna.name"
            ></mwc-textfield>

            <div class="row">
              <div class="column">
                <mwc-list>
                  <mwc-list-item
                    v-for="zome of dna.zomes"
                    :key="zome.name"
                    @click="selectedZomes[dna.name] = zome.name"
                  >
                    {{ zome.name }}
                  </mwc-list-item>
                </mwc-list>

                <mwc-button label="Add zome" @click="dna.zomes.push({ name: 'new-zome' })"></mwc-button>
              </div>

              <mwc-textfield
                label="Zome Name"
                class="text-input"
                :value="selectedZomes[dna.name].name"
                @input="selectedZomes[dna.name].name = $event.target.value"
                required
                autoValidate
              ></mwc-textfield>
            </div>
          </div>
        </mwc-card>

        <mwc-button @click="$emit('scaffoldApp', happ)" style="margin-top: 24px" label="Scaffold new app"></mwc-button>
      </div>
    </div>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { socket } from '../socket';
import { ClientEventType } from '@holochain/scaffolding-types';
import { generateWebHapp, HappDefinition } from '@holochain/scaffolding-generators';
import '@material/mwc-textfield';
import '@authentic/mwc-card';

export default {
  name: 'AppDefinitionBuilder',

  data(): { happ: HappDefinition; selectedZomes: { [dna: string]: string } } {
    return {
      selectedZomes: {
        'my-dna': 'my-zome',
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
  emits: ['scaffoldApp'],
};
</script>
<style scoped>
.row {
  display: flex;
  flex-direction: row;
}
.column {
  display: flex;
  flex-direction: column;
}

.center {
  justify-content: center;
  align-items: center;
}

.text-input {
  width: 424px;
  margin: 16px;
}
</style>