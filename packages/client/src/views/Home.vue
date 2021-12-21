<template>
  <div class="row" style="flex: 1">
    <div
      class="column"
      style="flex-basis: 500px; box-shadow: 1px 0px 4px grey; background-color: white; align-items: stretch"
    >
      <div class="column center" style="flex: 1">
        <img :src="require('../assets/holochain_logo.png')" class="my-3" contain height="200" width="200" />
        <h1>Holochain RAD Tools</h1>
        <mwc-button
          icon="bug_report"
          label="Report Issue"
          @click="open('https://github.com/holochain/scaffolding/issues')"
        >
        </mwc-button>
      </div>

      <div
        class="column"
        style="
          box-shadow: grey 1px 0px 4px;
          padding: 48px;
          background-color: #4720e3;
          align-items: center;
          color: white;
        "
      >
        <span class="secondary-title">Next Steps</span>
        <span class="documentation-item"><a href="https://developer.holochain.org/concepts/">Core Concepts</a></span>
        <span class="documentation-item"><a href="https://docs.rs/hdk">HDK Documentation</a></span>
        <span class="documentation-item"><a href="https://forum.holochain.org/">Holochain Forum</a></span>
        <span class="documentation-item"
          ><a href="https://github.com/holochain/happ-build-tutorial/">hApp Build Tutorial</a></span
        >
        <span class="documentation-item"
          ><a href="https://holochain-gym.github.io/developers/basic/zome-functions/">Holochain Gym Tutorial</a></span
        >
      </div>
    </div>
    <Scaffold style="flex: 1" class="column"></Scaffold>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Scaffold from '../components/Scaffold.vue';

export default defineComponent({
  name: 'Home',

  components: {
    Scaffold,
  },
  methods: {
    open(url: string) {
      window.open(url);
    },
  },
});
</script>
<style scoped>
.documentation-item {
  margin-top: 16px;
}

a {
  font-size: 20px;
}

a:link {
  color: white;
  background-color: transparent;
  text-decoration: none;
}

a:visited {
  color: #0dddd3;
  background-color: transparent;
  text-decoration: none;
}

a:hover {
  color: #0dddd3;
  background-color: transparent;
  text-decoration: underline;
}

a:active {
  color: #0dddd3;
  background-color: transparent;
  text-decoration: underline;
}
</style>