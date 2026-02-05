use super::super::*;
use crate::scaffold::{
    entry_type::definitions::{
        Cardinality, EntryTypeReference, FieldDefinition, FieldType, Referenceable,
    },
    web_app::template_type::TemplateType,
};

const INTEGRITY_ZOME_NAME: &str = "test_zome_integrity";

fn render_template(entry_type: EntryDefinition) -> String {
    let template_file_tree = TemplateType::Svelte.file_tree().unwrap();
    let h = build_handlebars(&template_file_tree).unwrap();
    let common_template_content = file_content(
        &template_file_tree,
        &PathBuf::from("entry-type/dnas/{{dna_role_name}}/zomes/coordinator/{{coordinator_zome_manifest.name}}/tests/common.rs.hbs"),
    )
    .unwrap();

    let data = ScaffoldEntryTypeData {
        app_name: "test-app".to_string(),
        dna_role_name: "test-dna".to_string(),
        integrity_zome_manifest: ZomeManifest {
            name: INTEGRITY_ZOME_NAME.into(),
            path: "zome_integrity".to_string(),
            dependencies: None,
            hash: None,
        },
        coordinator_zome_manifest: ZomeManifest {
            name: "test-zome".into(),
            path: "zome".to_string(),
            dependencies: None,
            hash: None,
        },
        entry_type,
        entry_type_ts_types: "",
        crud: Crud::default(),
        link_from_original_to_each_update: false,
    };
    h.render_template(&common_template_content, &data).unwrap()
}

fn expected_common(return_value: &str) -> String {
    format!(
        r#"use hdk::prelude::*;
use holochain::sweettest::{{SweetConductor, SweetZome}};
use {INTEGRITY_ZOME_NAME}::*;

pub async fn sample_test_entry(conductor: &SweetConductor, zome: &SweetZome) -> TestEntry {{
    {return_value}
}}

pub async fn create_test_entry(conductor: &SweetConductor, zome: &SweetZome) -> Record {{
    conductor
        .call(&zome, "create_test_entry", sample_test_entry(conductor, zome).await)
        .await
}}
"#
    )
}

#[test]
fn single_string() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::String,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Default::default(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_bool() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Bool,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Default::default(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_u8() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::U8,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Default::default(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_agent_pub_key() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::AgentPubKey,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: AgentPubKey::from_raw_36(vec![0; 36]),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_action_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: ActionHash::from_raw_36(vec![0; 36]),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_entry_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::EntryHash,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: EntryHash::from_raw_36(vec![0; 36]),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_dna_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::DnaHash,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: DnaHash::from_raw_36(vec![0; 36]),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_external_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ExternalHash,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: ExternalHash::from_raw_36(vec![0; 36]),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_timestamp() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Timestamp,
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Timestamp::now(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_enum() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Enum {
                label: "TestEnum".to_string(),
                variants: vec!["Variant1".to_string(), "Variant2".to_string()],
            },
            cardinality: Cardinality::Single,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);
    let expected_return_value = r#"TestEntry {
        test_field: TestEnum::Variant1,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn vector() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::String,
            cardinality: Cardinality::Vector,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Vec::new(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_action_hash_linked_from_action_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Single,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: false,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);
    let expected_return_value = r#"TestEntry {
        test_field: create_other_test_entry(conductor, zome).await.signed_action.hashed.hash,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_action_hash_linked_from_entry_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Single,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: true,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: create_other_test_entry(conductor, zome).await.signed_action.hashed.content.entry_hash().unwrap().clone(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn single_action_hash_linked_from_agent_pub_key() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Single,
            linked_from: Some(Referenceable::Agent {
                role: "perpetrator".to_string(),
            }),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);
    let expected_return_value = r#"TestEntry {
        test_field: zome.cell_id().agent_pubkey().clone(),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_action_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_agent_pub_key() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::AgentPubKey,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_bool() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Bool,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_dna_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::DnaHash,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_enum() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Enum {
                label: "TestEnum".to_string(),
                variants: vec!["Variant1".to_string()],
            },
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_timestamp() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::Timestamp,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_string() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::String,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_external_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ExternalHash,
            cardinality: Cardinality::Option,
            linked_from: None,
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_linked_from_self_reference() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Option,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "test_entry".to_string(),
                reference_entry_hash: false,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: None,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_linked_from_agent_pub_key() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::AgentPubKey,
            cardinality: Cardinality::Option,
            linked_from: Some(Referenceable::Agent {
                role: "NailClipper".to_string(),
            }),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Some(zome.cell_id().agent_pubkey().clone()),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_linked_from_action_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Option,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: false,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Some(create_other_test_entry(conductor, zome).await.signed_action.hashed.hash),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn option_linked_from_entry_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Option,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: true,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Some(create_other_test_entry(conductor, zome).await.signed_action.hashed.content.entry_hash().unwrap().clone()),
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn vector_action_hash_linked_from_action_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Vector,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: false,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: vec![create_other_test_entry(conductor, zome).await.signed_action.hashed.hash],
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn vector_action_hash_linked_from_entry_hash() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![FieldDefinition {
            field_name: "test_field".to_string(),
            field_type: FieldType::ActionHash,
            cardinality: Cardinality::Vector,
            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: "other_test_entry".to_string(),
                reference_entry_hash: true,
            })),
            widget: None,
        }],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: vec![create_other_test_entry(conductor, zome).await.signed_action.hashed.content.entry_hash().unwrap().clone()],
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}

#[test]
fn two_single_fields() {
    let entry_type = EntryDefinition {
        name: "test_entry".to_string(),
        fields: vec![
            FieldDefinition {
                field_name: "test_field".to_string(),
                field_type: FieldType::String,
                cardinality: Cardinality::Single,
                linked_from: None,
                widget: None,
            },
            FieldDefinition {
                field_name: "test_field_2".to_string(),
                field_type: FieldType::ActionHash,
                cardinality: Cardinality::Single,
                linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                    entry_type: "other_test_entry".to_string(),
                    reference_entry_hash: false,
                })),
                widget: None,
            },
        ],
        reference_entry_hash: false,
    };
    let rendered_common = render_template(entry_type);

    let expected_return_value = r#"TestEntry {
        test_field: Default::default(),
        test_field_2: create_other_test_entry(conductor, zome).await.signed_action.hashed.hash,
    }"#;
    pretty_assertions::assert_str_eq!(rendered_common, expected_common(expected_return_value));
}
