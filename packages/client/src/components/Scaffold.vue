<template>
  <div class="column" style="align-items: center">
    <div style="width: 800px">
      <AppDefinitionBuilder @scaffold-app="generateFileChanges($event)"></AppDefinitionBuilder>
    </div>
  </div>

  <mwc-dialog ref="dialog" heading="Preview" scrimClickAction="" escapeKeyAction="">
    <div class="column">
      <span>This will create the following structure in the directory: {{ currentDir }}</span>

      <ui5-tree>
        <FileNode :fileTree="fileChanges"> </FileNode>
      </ui5-tree>
    </div>

    <mwc-button slot="secondaryAction" dialogAction="close" label="Cancel"></mwc-button>
    <mwc-button slot="primaryAction" dialogAction="close" @click="scaffoldApp()" label="Yes"></mwc-button>
  </mwc-dialog>
  <mwc-dialog ref="helpdialog" heading="App Scaffolded!">
    <div class="column">
      <span
        >If you haven't yet, <b><a href="https://nixos.org/download.html">install nix-shell</a>.</b></span
      >
      <span>Run this to get started:</span>
      <code class="language-bash" style="width: 800px;">
        cd {{ currentDir }}/{{ happName }}
        
        nix-env -iA cachix -f https://cachix.org/api/v1/install
        
        nix-shell && npm install
      </code>
    </div>
    <mwc-button slot="primaryAction" dialogAction="close" label="Ok"></mwc-button>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { socket } from '../socket';
import { ClientEventType } from '@holochain/scaffolding-types';
import { FileChanges, FileChangesType, generateWebHapp, HappDefinition } from '@holochain/scaffolding-generators';
import AppDefinitionBuilder from './AppDefinitionBuilder.vue';
import FileNode from './FileNode.vue';
import type { Dialog } from '@material/mwc-dialog';

export default defineComponent({
  name: 'Scaffold',
  components: {
    AppDefinitionBuilder,
    FileNode,
  },
  data(): { currentDir: string | undefined; fileChanges: FileChanges[] | undefined; happName: string | undefined } {
    return {
      currentDir: undefined,
      fileChanges: undefined,
      happName: undefined,
    };
  },
  async mounted() {
    socket.emit(ClientEventType.ReadDir, (dir: { dirPath: string }) => (this.currentDir = dir.dirPath));
  },
  methods: {
    scaffoldApp(): void {
      socket.emit(ClientEventType.ApplyChanges, [
        {
          type: FileChangesType.InDir,
          dirName: this.happName,
          changes: this.fileChanges,
        },
      ]);
      (this.$refs.helpdialog as Dialog).show();
    },
    generateFileChanges(happ: HappDefinition) {
      this.fileChanges = generateWebHapp(happ);
      this.happName = happ.name;
      (this.$refs.dialog as Dialog).show();
    },
  },
});
</script>
<style scoped>
.text-input {
  width: 424px;
  margin: 16px;
}
</style>