<template>
  <div class="column" style="flex: 1">
    <mwc-top-app-bar style="flex: 1; display: flex">
      <img
        :src="require('../assets/holochain_logo.png')"
        class="my-3"
        contain
        height="32"
        width="32"
        slot="navigationIcon"
      />
      <div slot="title">Scaffold New hApp</div>
      <mwc-button
        icon="bug_report"
        label="Report Issue"
        slot="actionItems"
        style="--mdc-theme-primary: white"
        @click="open('https://github.com/holochain/scaffolding/issues')"
      >
      </mwc-button>

      <div style="width: 100vw; height: 100%; flex: 1; display: flex">
        <Scaffold></Scaffold>
      </div>
    </mwc-top-app-bar>
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