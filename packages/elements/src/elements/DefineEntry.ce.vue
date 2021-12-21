<template>
  <div style="display: flex; flex-direction: column; flex: 1; margin: 16px">
    <span style="font-size: 22px">Entry Definition: {{ entryDef.name }}</span>

    <mwc-textfield
      label="Entry Definition Id"
      @focus="entryDefValidity($event.target)"
      @input="$event.target.validity.valid && setEntryDefId($event.target.value)"
      required
      :value="entryDef.name"
      outlined
      helper="Has to be unique, and snake_case"
      autoValidate
      style="width: 424px; margin-top: 16px;"
    ></mwc-textfield>

    <div style="display: flex; flex-direction: row; flex: 1">
      <mwc-textarea
        outlined
        autoValidate
        helper="Has to be a valid JSON object"
        label="JSON Sample"
        ref="json-field"
        @focus="entryDefSampleValidity($event.target)"
        @input="$event.target.validity.valid && setEntryDefSample($event.target.value)"
        style="flex: 1"
      >
      </mwc-textarea>

      <div style="display: flex; flex-direction: column; flex: 1; margin-left: 24px">
        <span style="font-size: 18px">CRUD</span>
        <mwc-formfield label="Read">
          <mwc-checkbox
            :checked="entryDef.read"
            @change="
              entryDef.read = $event.target.checked;
              emitChanged();
            "
          ></mwc-checkbox>
        </mwc-formfield>
        <mwc-formfield label="Create">
          <mwc-checkbox
            :checked="entryDef.create"
            @change="
              entryDef.create = $event.target.checked;
              emitChanged();
            "
          ></mwc-checkbox>
        </mwc-formfield>
        <mwc-formfield label="Update">
          <mwc-checkbox
            :checked="entryDef.update"
            @change="
              entryDef.update = $event.target.checked;
              emitChanged();
            "
          ></mwc-checkbox>
        </mwc-formfield>
        <mwc-formfield label="Delete">
          <mwc-checkbox
            :checked="entryDef.delete"
            @change="
              entryDef.delete = $event.target.checked;
              emitChanged();
            "
          ></mwc-checkbox>
        </mwc-formfield>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { EntryDefinition } from '@holochain/rad-definitions';
import { isSnakeCase } from '@holochain/rad-generators';

export default defineComponent({
  name: 'DefineEntry',

  props: {
    entryDef: { type: Object as PropType<EntryDefinition>, required: true },
    otherEntryDefsNames: { type: Array, required: true },
  },
  mounted() {
    this.onEntryDefChanged();
  },
  watch: {
    entryDef: function () {
      this.onEntryDefChanged();
    },
  },
  methods: {
    onEntryDefChanged() {
      const sampleField = this.$refs['json-field'] as any;
      sampleField.value = JSON.stringify(this.entryDef.sample, null, 2);
    },
    entryDefSampleValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }

        try {
          JSON.parse(newValue);
          textfield.setCustomValidity('');
          return {
            valid: true,
          };
        } catch (e) {
          textfield.setCustomValidity('The entry sample must be a valid JSON object');
          return {
            valid: false,
          };
        }
      };
    },
    entryDefValidity(textfield: TextField) {
      textfield.validityTransform = (newValue, nativeValidity) => {
        if (newValue === '') {
          textfield.setCustomValidity('Must not be empty');
          return {
            valid: false,
          };
        }

        if (!isSnakeCase(newValue)) {
          textfield.setCustomValidity('The entry def id must be snake_case');
          return {
            valid: false,
          };
        }

        if (this.otherEntryDefsNames.find(name => name === newValue)) {
          textfield.setCustomValidity('The entry_def_id has to be unique in this zome');
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
    setEntryDefId(newValue: string) {
      this.entryDef.name = newValue;
      this.emitChanged();
    },
    setEntryDefSample(newValue: string) {
      this.entryDef.sample = JSON.parse(newValue);
      this.emitChanged();
    },
    emitChanged() {
      this.$emit('entry-def-changed', this.entryDef);
    },
  },
});
</script>