<template>
  <ui5-tree-item
    v-for="(file, index) in fileTree"
    :key="index"
    :text="file.type === 'InDir' ? file.dirName : file.fileName"
  >
    <FileNode v-if="file.type === 'InDir'" :fileTree="file.changes"></FileNode>
  </ui5-tree-item>
</template>

<script lang="ts">
import { FileChanges, FileChangesType } from '@holochain/scaffolding-generators';

export default {
  name: 'FileNode',

  props: {
    fileTree: Array,
  },
};
</script>