<template>
  <ParentDetail
    :parent="happ"
    :children="happ.dnas"
    parentLabel="Happ"
    childrenLabel="Dna"
    @parent-changed="emitChanged()"
    @add-child="addDna()"
    @delete-child="deleteDna($event)"
    @child-selected="selectDna($event)"
  ></ParentDetail>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { HappDefinition } from '@holochain/rad-definitions';
import { newDna } from '../utils';
import ParentDetail from './ParentDetail.vue';

export default defineComponent({
  name: 'DefineHapp',
  components: { ParentDetail },

  props: {
    happ: {
      type: Object as PropType<HappDefinition>,
      required: true,
    },
  },
  data(): {
    dnaCount: number;
  } {
    return {
      dnaCount: 1,
    };
  },
  methods: {
    addDna() {
      const name = `dna_${this.dnaCount++}`;
      this.happ.dnas.push(newDna(name));

      this.$emit('dna-added', this.happ.dnas.length - 1);
      this.emitChanged();
    },
    deleteDna(dnaIndex: number) {
      this.happ.dnas.splice(dnaIndex, 1);
      this.$emit('dna-deleted', dnaIndex);
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('happ-changed', this.happ);
    },
    selectDna(dnaIndex: number) {
      this.$emit('dna-selected', dnaIndex);
    },
  },
});
</script>