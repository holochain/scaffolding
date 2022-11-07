
export interface {{pascal_case entry_type.name}} { {{#each entry_type.fields}}
  {{snake_case @key}}: {{> (concat field_type.type "/type") }};
{{/each}}
}
