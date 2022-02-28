<template>
  <ui5-tree-item v-for="([name, node], index) of children" :key="index" :text="name">
    <DirectoryTree v-if="node.type === ScNodeType.Directory" :directory="node"></DirectoryTree>
  </ui5-tree-item>
</template>

<script lang="ts">
import { ScDirectory, ScNode, ScNodeType } from '@source-craft/types';
import { defineComponent, PropType } from 'vue';

export default defineComponent({
  name: 'DirectoryTree',

  props: {
    directory: {
      type: Object as PropType<ScDirectory>,
      required: true,
    },
  },
  computed: {
    children() {
      return Object.entries(this.directory.children).sort(
        ([_, node1]: [string, ScNode], [__, node2]: [string, ScNode]) => {
          if (node1.type === ScNodeType.Directory) return -1;
          if (node2.type === ScNodeType.Directory) return 1;
          return -1;
        },
      );
    },
  },
  setup() {
    return {
      ScNodeType,
    };
  },
});
</script>