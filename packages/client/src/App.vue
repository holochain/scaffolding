<template>
  <router-view />
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'App',
});
</script>
<style>
body {
  font-family: Mulish, sans-serif;
  background-color: #ededed;
}

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

.flex-scrollable-parent {
  position: relative;
  display: flex;
  flex: 1;
}
.flex-scrollable-container {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}
.flex-scrollable-x {
  max-width: 100%;
  overflow-x: auto;
}
.flex-scrollable-y {
  max-height: 100%;
  overflow-y: auto;
}
</style>
