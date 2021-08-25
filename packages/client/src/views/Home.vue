<template>
  <scaffold />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Scaffold from '../components/Scaffold.vue';

export default defineComponent({
  name: 'Home',

  components: {
    Scaffold,
  },
});
</script>
