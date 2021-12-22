<template>
  <div class="row" style="flex: 1">
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