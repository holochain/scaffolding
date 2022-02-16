<template>
  <div style="display: flex; flex-direction: column; flex: 1">
    <div style="display: flex; flex-direction: row">
      <mwc-textfield
        label="Zome Name"
        style="width: 424px; margin: 16px"
        required
        autoValidate
        ref="zome-name"
        outlined
        validationMessage="Must not be empty"
        helper="Has to be unique within the Dna, and snake_case"
        @focus="zomeValidity($event.target)"
        @input="setZomeName($event.target)"
      ></mwc-textfield>

      <span style="flex: 1"></span>

      <slot></slot>
    </div>

    <div style="display: flex; flex-direction: row">
      <span style="flex: 1; font-size: 18px; margin-left: 16px">Entry Defs</span>
    </div>
    <span style="height: 1px; width: 100%; background-color: lightgrey; margin-top: 8px"></span>
    <div style="display: flex; flex-direction: row; flex: 1">
      <div style="display: flex; flex-direction: column; flex-basis: 300px">
        <div style="position: relative; display: flex; flex: 1">
          <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0">
            <div style="max-height: 100%; overflow-y: auto">
              <mwc-list activatable>
                <mwc-list-item
                  v-for="(entryDef, entryDefIndex) of zome.entry_defs"
                  :key="entryDefIndex"
                  graphic="icon"
                  :activated="selectedEntryDefIndex === entryDefIndex"
                  @click="selectedEntryDefIndex = entryDefIndex"
                  style="flex: 1"
                >
                  <mwc-icon slot="graphic">sticky_note_2</mwc-icon>
                  {{ entryDef.name }}
                </mwc-list-item>
              </mwc-list>
            </div>
          </div>
        </div>

        <mwc-button icon="add" label="Add Entry Def" @click="addEntryDef()"></mwc-button>
      </div>

      <span style="height: 100%; width: 1px; background-color: lightgrey"></span>

      <DefineEntry
        v-if="selectedEntryDef"
        :key="selectedEntryDefIndex"
        :entryDef="selectedEntryDef"
        :otherEntryDefsNames="otherEntryDefsNames"
        @entry-def-changed="emitChanged()"
        style="margin-left: 16px; margin-bottom: 8px"
      >
        <mwc-button
          label="Remove Entry Def"
          :disabled="zome.entry_defs.length < 2"
          icon="delete"
          @click="deleteEntryDef()"
          style="margin: 8px; --mdc-theme-primary: black"
        ></mwc-button>
      </DefineEntry>
      <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
        <span style="opacity: 0.6">Select an entry def </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { isSnakeCase } from '@holochain-scaffolding/patcher';
import { ZomeDefinition, newEntryDef, newZomeDef } from '@holochain-scaffolding/definitions';
import DefineEntry from './DefineEntry.ce.vue';

export default defineComponent({
  name: 'DefineZome',

  components: {
    DefineEntry,
  },

  props: {
    zome: { type: Object as PropType<ZomeDefinition>, required: false, default: newZomeDef() },
    otherZomesNames: { type: Array, required: false, default: [] },
  },
  data(): { entryDefCount: number; selectedEntryDefIndex: number } {
    return {
      entryDefCount: 1,
      selectedEntryDefIndex: -1,
    };
  },
  computed: {
    otherEntryDefsNames() {
      return this.zome.entry_defs
        .filter((_, index) => index !== this.selectedEntryDefIndex)
        .map(entryDef => entryDef.name);
    },
    selectedEntryDef() {
      if (this.selectedEntryDefIndex === -1) return undefined;
      else return this.zome.entry_defs[this.selectedEntryDefIndex];
    },
  },
  mounted() {
    this.updateZomeName();
  },
  watch: {
    zome() {
      this.updateZomeName();
      this.selectedEntryDefIndex = -1;
    },
  },
  methods: {
    updateZomeName() {
      // setTimeout is a workaround for the mwc-textfield label bug
      setTimeout(() => {
        const field = this.$refs['zome-name'] as TextField;
        field.value = this.zome.name;
      }, 1);
    },
    addEntryDef() {
      const name = `entry_def_${this.entryDefCount++}`;
      this.zome.entry_defs.push(newEntryDef(name));
      this.selectedEntryDefIndex = this.zome.entry_defs.length - 1;
      this.emitChanged();
    },
    deleteEntryDef() {
      this.zome.entry_defs.splice(this.selectedEntryDefIndex, 1);

      this.selectedEntryDefIndex--;
      if (this.selectedEntryDefIndex < 0) this.selectedEntryDefIndex = 0;

      this.emitChanged();
    },
    zomeValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }

        if (!isSnakeCase(newValue)) {
          textfield.setCustomValidity('The zome name must be snake_case');
          return {
            valid: false,
          };
        }

        if (this.otherZomesNames.find(name => name === newValue)) {
          textfield.setCustomValidity('The zome name has to be unique');
          return {
            valid: false,
          };
        }
        textfield.setCustomValidity('');
        return {
          valid: true,
        };
      };
    },
    setZomeName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.zome.name = textfield.value;
      }
      this.emitChanged();
    },
    emitChanged() {
      this.$forceUpdate();
      this.$emit('zome-changed', this.zome);
    },
  },
});
</script>