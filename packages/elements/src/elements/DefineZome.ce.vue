<template>
  <ParentDetail
    :parent="zome"
    :children="zome.entry_defs"
    parentLabel="Zome"
    childrenLabel="Entry Def"
    :siblingsNames="otherZomesNames"
    :snakeCaseRequired="true"
    @parent-changed="emitChanged()"
    @add-child="addEntryDef()"
    @delete-child="deleteEntryDef($event)"
    @child-selected="selectEntryDef($event)"
  ></ParentDetail>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { isSnakeCase } from '@holochain/rad-generators';
import { ZomeDefinition } from '@holochain/rad-definitions';
import { newEntryDef } from '../utils';
import ParentDetail from './ParentDetail.vue';

export default defineComponent({
  name: 'DefineZome',
  components: { ParentDetail },

  props: {
    zome: { type: Object as PropType<ZomeDefinition>, required: true },
    otherZomesNames: { type: Array, required: true },
  },
  data(): { entryDefCount: number } {
    return {
      entryDefCount: 1,
    };
  },
  methods: {
    selectEntryDef(entryDefIndex: number) {
      this.$emit('entry-def-selected', entryDefIndex);
    },
    addEntryDef() {
      const name = `entry_def_${this.entryDefCount++}`;
      this.zome.entry_defs.push(newEntryDef(name));
      this.$emit('entry-def-added', this.zome.entry_defs.length - 1);
      this.emitChanged();
    },
    deleteEntryDef(entryDefIndex: number) {
      this.zome.entry_defs.splice(entryDefIndex, 1);
      this.$emit('entry-def-deleted');
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
    setZomeName(newValue: string) {
      this.zome.name = newValue;
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('zome-changed', this.zome);
    },
  },
});
</script>