<template>
  <div style="display: flex; flex-direction: column; flex: 1">
    <div style="display: flex; flex-direction: row">
      <mwc-textfield
        label="Entry Definition Id"
        @focus="entryDefValidity($event.target)"
        @input="$event.target.validity.valid && setEntryDefId($event.target.value)"
        required
        :value="entryDef.name"
        outlined
        helper="Has to be unique within the zome, and snake_case"
        autoValidate
        style="width: 424px; margin-top: 16px"
      ></mwc-textfield>
      <span style="flex: 1"></span>
      <slot></slot>
    </div>

    <div style="display: flex; flex-direction: row; flex: 1">
      <div style="display: flex; flex-direction: column; flex-basis: 250px">
        <span style="font-size: 18px">CRUD Handlers</span>
        <mwc-formfield label="Create" style="opacity: 0.4">
          <mwc-checkbox
            :checked="entryDef.create"
            :disabled="true"
            @change="
              entryDef.create = $event.target.checked;
              emitChanged();
            "
          ></mwc-checkbox>
        </mwc-formfield>
        <mwc-formfield label="Read">
          <mwc-checkbox
            :checked="entryDef.read"
            @change="
              entryDef.read = $event.target.checked;
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

      <div style="position: relative; display: flex; flex: 1">
        <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0">
          <div style="max-height: 100%; overflow-y: auto">
            <craft-fields
              :fields="entryDef.typeDefinition.fields"
              style="flex: 1"
              @change="setFields($event.target.value)"
            ></craft-fields>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import type { TextField } from '@material/mwc-textfield';
import { EntryDefinition, holochainEntryRustTypeGenerator, newEntryDef } from '@holochain-scaffolding/definitions';
import { isSnakeCase } from '@holochain-scaffolding/patcher';
import { defaultTsGenerator, FieldDefinition, ProgrammingLanguages } from '@typecraft/type-definition';

export default defineComponent({
  name: 'DefineEntry',

  props: {
    entryDef: { type: Object as PropType<EntryDefinition>, required: false, default: newEntryDef() },
    otherEntryDefsNames: { type: Array, required: false, default: [] },
  },
  mounted() {
    this.onEntryDefChanged();
  },
  watch: {
    entryDef: function () {
      this.onEntryDefChanged();
    },
    entryDefIndex: function () {
      this.onEntryDefChanged();
    },
  },
  methods: {
    onEntryDefChanged() {},
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

        if (this.otherEntryDefsNames.includes(newValue)) {
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
    setFields(fields: Array<FieldDefinition<any>>) {
      this.entryDef.typeDefinition.fields = fields;
      this.emitChanged();
    },
    emitChanged() {
      (this.entryDef.typeDefinition.generators = {
        [ProgrammingLanguages.Typescript]: defaultTsGenerator(
          this.entryDef.name,
          this.entryDef.typeDefinition.fields!,
        ),
        [ProgrammingLanguages.Rust]: holochainEntryRustTypeGenerator(
          this.entryDef.name,
          this.entryDef.typeDefinition.fields!,
        ),
      }),
        this.$emit('entry-def-changed', this.entryDef);
    },
  },
});
</script>