<template>
  <ParentDetail
    :parent="dna"
    :children="dna.zomes"
    parentLabel="Dna"
    childrenLabel="Zome"
    :siblingsNames="otherDnasNames"
    :snakeCaseRequired="true"
    @parent-changed="emitChanged()"
    @add-child="addZome()"
    @delete-child="deleteZome($event)"
    @child-selected="selectZome($event)"
  ></ParentDetail>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { DnaDefinition, HappDefinition } from '@holochain/rad-definitions';
import { newZome } from '../utils';
import ParentDetail from './ParentDetail.vue';

export default defineComponent({
  name: 'DefineDna',
  components: { ParentDetail },

  props: {
    dna: { type: Object as PropType<DnaDefinition>, required: true },
    otherDnasNames: { type: Array, required: false, default: [] },
  },
  data(): {
    zomeCount: number;
  } {
    return {
      zomeCount: 1,
    };
  },
  methods: {
    selectZome(zomeIndex: number) {
      this.$emit('zome-selected', zomeIndex);
    },
    addZome() {
      const name = `zome_${this.zomeCount++}`;
      const zomes = this.dna.zomes;
      zomes.push(newZome(name));
      this.$emit('zome-added', this.dna.zomes.length - 1);
      this.emitChanged();
    },
    deleteZome(zomeIndex: number) {
      this.dna.zomes.splice(zomeIndex, 1);
      this.$emit('zome-deleted', zomeIndex);
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('dna-changed', this.dna);
    },
  },
});
</script>