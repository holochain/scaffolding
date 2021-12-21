<template>
  <div style="display: flex; flex-direction: row; flex: 1">
    <div style="display: flex; flex-direction: column; flex: 1">
      <span style="font-size: 18px">{{ parentLabel }}: {{ parent.name }}</span>
      <mwc-textfield
        :label="`${parentLabel} Name`"
        style="width: 424px; margin-top: 16px"
        required
        autoValidate
        :value="parent.name"
        outlined
        validationMessage="Must not be empty"
        @focus="nameValidity($event.target)"
        @input="setName($event.target)"
      ></mwc-textfield>

      <slot name="additionalProperty"></slot>
    </div>

    <div style="display: flex; flex-direction: column; flex: 1">
      <div style="display: flex; flex-direction: row; flex: 1; align-items: center; justify-content: center">
        <span style="flex: 1; font-size: 16px">{{ childrenLabel }}</span>
        <mwc-button icon="add" :label="`Add ${childrenLabel}`" @click="$emit('add-child')"></mwc-button>
      </div>
      <mwc-list>
        <div
          style="display: flex; flex-direction: row; flex: 1"
          v-for="(child, childIndex) of children"
          :key="child.name"
        >
          <mwc-list-item @click="$emit('child-selected', childIndex)" style="flex: 1">
            {{ child.name }}
          </mwc-list-item>

          <mwc-icon-button
            :disabled="children.length < 2"
            @click="$emit('delete-child', childIndex)"
            icon="delete"
          ></mwc-icon-button>
        </div>
      </mwc-list>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { Nameable } from './types';
import { isSnakeCase } from '@holochain/rad-generators';

export default defineComponent({
  name: 'ParentDetail',

  props: {
    parent: {
      type: Object as PropType<Nameable>,
      required: true,
    },
    siblingsNames: {
      type: Object as PropType<Array<string>>,
      required: false,
      default: [],
    },
    children: {
      type: Object as PropType<Array<Nameable>>,
      required: true,
    },
    parentLabel: {
      type: String,
      required: true,
    },
    childrenLabel: {
      type: String,
      required: true,
    },
    snakeCaseRequired: {
      type: Boolean,
      required: false,
      default: false,
    },
  },
  methods: {
    nameValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }
        if (this.snakeCaseRequired && !isSnakeCase(newValue)) {
          textfield.setCustomValidity('The zome name must be snake_case');
          return {
            valid: false,
          };
        }

        if (this.siblingsNames.find(name => name === newValue)) {
          textfield.setCustomValidity('The DNA name has to be unique');
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
    setName(textfield: TextField) {
      if (textfield.validity.valid) {
        this.parent.name = textfield.value;
      }
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('parent-changed', this.parent);
    },
  },
});
</script>