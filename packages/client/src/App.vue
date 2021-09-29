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
html,
body,
#app {
  height: 100%;
  width: 100%;
  margin: 0;
}

#app {
  display: flex;
}

.secondary-title {
  font-size: 24px;
}

.tertiary-title {
  font-size: 18px;
}

body {
  font-family: Mulish, sans-serif;
  background-color: #ededed;
}
:root {
  font-family: Mulish, sans-serif;
  --mdc-theme-primary: #4720e3;
  --mdc-theme-secondary: #0dddd3;
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
