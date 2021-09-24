<template>
  <v-container>
    <div class="column center">
      <v-img :src="require('../assets/holochain_logo.svg')" class="my-3" contain height="200" width="200" />

      <div class="column center">
        <h1>Welcome to the Holochain RAD tools</h1>
        <AppDefinitionBuilder @scaffold-app="$event"></AppDefinitionBuilder>
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
import AppDefinitionBuilder from './AppDefinitionBuilder.vue';
import '@authentic/mwc-card';

export default {
  name: 'Scaffold',
  components: {
    AppDefinitionBuilder
  },
  methods: {
    scaffoldApp(happ: HappDefinition): void {
      socket.emit(ClientEventType.ApplyChanges, generateWebHapp(happ));
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