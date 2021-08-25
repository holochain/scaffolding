<template>
  <v-container>
    <div class="column center">
      <v-img :src="require('../assets/holochain_logo.svg')" class="my-3" contain height="200" width="200" />

      <div class="column center">
        <h1>Welcome to the Holochain RAD tools</h1>
        <mwc-textfield label="hApp Name" ref="happName" class="text-input" :value="'sample-happ'"></mwc-textfield>
        <mwc-textfield label="DNA Name" ref="dnaName" class="text-input" :value="'sample-dna'"></mwc-textfield>
        <mwc-textfield label="Zome Name" ref="zomeName" class="text-input" :value="'sample-zome'"></mwc-textfield>

        <v-btn @click="scaffold()" style="margin-top: 24px">Scaffold new hApp</v-btn>
      </div>
    </div>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { socket } from '../socket';
import { ClientEventType } from '@holochain/create-types';
import { generateHapp } from '@holochain/create-generators';
import '@material/mwc-textfield';

export default {
  name: 'Scaffold',

  data(): { happName: string; dnaName: string; zomeName: string } {
    return {
      happName: 'sample-happ',
      dnaName: 'sample-dna',
      zomeName: 'sample-zome',
    };
  },
  methods: {
    scaffold(): void {
      socket.emit(
        ClientEventType.ApplyChanges,
        generateHapp(
          (this as any).$refs.happName.value,
          (this as any).$refs.dnaName.value,
          (this as any).$refs.zomeName.value,
        ),
      );
    },
  },
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