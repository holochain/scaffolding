<template>
  <div class="column" style="align-items: center">
    <div style="width: 800px">
      <AppDefinitionBuilder @scaffold-app="generateFileChanges($event)"></AppDefinitionBuilder>
    </div>
  </div>

  <mwc-dialog ref="dialog" heading="Preview" scrimClickAction="" escapeKeyAction="">
    <div class="column">
      <span
        >This will <b>create a new folder {{ happName }}</b> in the directory where you executed the CLI with the
        following structure.</span
      >

      <ui5-tree>
        <FileNode :fileTree="fileChanges"> </FileNode>
      </ui5-tree>
    </div>

    <mwc-button slot="secondaryAction" dialogAction="close" label="Cancel"></mwc-button>
    <mwc-button slot="primaryAction" dialogAction="close" @click="scaffoldApp()" label="Create"></mwc-button>
  </mwc-dialog>
  <mwc-dialog ref="helpdialog" heading="App Scaffolded!">
    <div v-if="!settingUp" class="column">
      <h3>Manual Setup</h3>
      <span
        >If you haven't yet, <b><a href="https://nixos.org/download.html">install nix-shell</a>.</b></span
      >
      <span>Run this to get started:</span>
      <pre><code class="language-bash" style="word-break: break-all;">cd {{ currentDir }}/{{ happName }}
nix-env -iA cachix -f https://cachix.org/api/v1/install
cachix use holochain-ci
nix-shell
npm install</code></pre>
      <span>After that, you can safely close this window.</span>
      <span style="margin-top: 16px">OR</span>
      <h3>Automatic Setup</h3>
      <span>You can automatically setup your app by selecting "SETUP AND EXIT".</span>
    </div>
    <div v-else>
      <span
        >You can safely close this window now, and wait for the automatic setup to complete in the terminal
        window.</span
      >
    </div>
    <mwc-button slot="secondaryAction" dialogAction="close" label="Close" v-if="!settingUp"></mwc-button>
    <mwc-button
      slot="primaryAction"
      @click="setup()"
      :label="settingUp ? 'Setting up...' : 'Setup and Exit'"
      :disabled="settingUp"
    ></mwc-button>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { socket } from '../socket';
import { ClientEventType } from '@holochain/scaffolding-types';
import { FileChanges, FileChangesType, generateWebHapp, HappDefinition } from '@holochain/scaffolding-generators';
import AppDefinitionBuilder from './AppDefinitionBuilder.vue';
import FileNode from './FileNode.vue';
import { getUiTemplate, replaceText } from '../utils';
import type { Dialog } from '@material/mwc-dialog';

export default defineComponent({
  name: 'Scaffold',
  components: {
    AppDefinitionBuilder,
    FileNode,
  },
  data(): {
    settingUp: boolean;
    currentDir: string | undefined;
    fileChanges: FileChanges[] | undefined;
    happName: string | undefined;
  } {
    return {
      settingUp: false,
      currentDir: undefined,
      fileChanges: undefined,
      happName: undefined,
    };
  },
  async mounted() {
    socket.emit(ClientEventType.ReadDir, (dir: { dirPath: string }) => (this.currentDir = dir.dirPath));
  },
  methods: {
    setup() {
      this.settingUp = true;
      socket.emit(ClientEventType.AutomaticSetup, this.happName);
    },
    close() {
      socket.emit(ClientEventType.Exit);
    },
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
    async generateFileChanges({ happ, uiTemplate }: { happ: HappDefinition; uiTemplate: string }) {
      const uiTemplateChanges = await getUiTemplate(uiTemplate, text =>
        replaceText(text, {
          installedAppId: happ.name,
          zomeName: happ.dnas[0].zomes[0].name,
        }),
      );

      const uiChanges: FileChanges = {
        type: FileChangesType.InDir,
        dirName: 'ui',
        changes: uiTemplateChanges,
      };

      this.fileChanges = [...generateWebHapp(happ), uiChanges];
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