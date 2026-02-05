use crate::{
    file_tree::{dir_content, file_content, file_exists, FileTree},
    scaffold::{
        entry_type::definitions::{Cardinality, EntryTypeReference, Referenceable},
        web_app::template_type::TemplateType,
    },
    templates::{
        build_handlebars,
        link_type::{scaffold_link_type_templates, ScaffoldLinkTypeData},
        ScaffoldedTemplate,
    },
};
use build_fs_tree::{dir, file};
use convert_case::{Case, Casing};
use holochain_types::dna::ZomeManifest;
use pluralizer::pluralize;
use std::{path::PathBuf, str::FromStr};

fn scaffold_link_template(from_role: &str, to: &Option<Referenceable>) -> ScaffoldedTemplate {
    let app_file_tree: FileTree = dir! {
        "dnas" => dir! {
            "test-dna" => dir! {
                "zomes" => dir! {
                    "coordinator" => dir! {
                        "test-zome" => dir! {
                            "src" => dir! {
                                "lib.rs" => file!("")
                            }
                        }
                    }
                }
            }
        }
    };
    let template_file_tree = TemplateType::Svelte.file_tree().unwrap();
    scaffold_link_type_templates(
        app_file_tree,
        &template_file_tree,
        "test-app",
        "test-dna",
        &ZomeManifest {
            name: "test-zome".into(),
            hash: None,
            path: "".to_string(),
            dependencies: None,
        },
        "test-link",
        &Referenceable::Agent {
            role: from_role.to_string(),
        },
        to,
        false,
        None,
        true,
        false,
    )
    .unwrap()
}

#[test]
fn no_link_type_test_file_is_generated_without_link_target() {
    let scaffolded_template = scaffold_link_template("frommer", &None);
    assert!(dir_content(
        &scaffolded_template.file_tree,
        PathBuf::from_str(&format!("dnas/test-dna/zomes/coordinator/test-zome/tests/"))
            .unwrap()
            .as_path()
    )
    .unwrap()
    .is_empty());
}

#[test]
fn link_type_test_file_is_generated() {
    let from_role = "frommer";
    let to_role = "toer";
    let scaffolded_template = scaffold_link_template(
        from_role,
        &Some(Referenceable::Agent {
            role: to_role.to_string(),
        }),
    );
    let to_role_plural = pluralize(to_role, 2, false);
    assert!(
        file_exists(
            &scaffolded_template.file_tree,
            PathBuf::from_str(&format!(
            "dnas/test-dna/zomes/coordinator/test-zome/tests/{from_role}_to_{to_role_plural}.rs"
        ))
            .unwrap()
            .as_path()
        ),
        "Expected test file doesn't exist, dir structure: {:?}",
        scaffolded_template.file_tree
    );
}

fn render_template(
    from: &Referenceable,
    to: &Referenceable,
    delete: bool,
    bidirectional: bool,
) -> String {
    let template_file_tree = TemplateType::Svelte.file_tree().unwrap();
    let h = build_handlebars(&template_file_tree).unwrap();
    let common_template_content = file_content(
        &template_file_tree,
        &PathBuf::from("link-type/dnas/{{dna_role_name}}/zomes/coordinator/{{coordinator_zome_manifest.name}}/tests/{{#if to_referenceable}}{{snake_case from_referenceable.name}}_to_{{snake_case (plural to_referenceable.name)}}.rs{{¡if}}.hbs"),
    )
    .unwrap();
    let bidirectional = bidirectional.then_some("bidirectional");

    let data = ScaffoldLinkTypeData {
        app_name: "test-app",
        dna_role_name: "test-dna",
        coordinator_zome_manifest: ZomeManifest {
            name: "test-zome".into(),
            path: "zome".to_string(),
            dependencies: None,
            hash: None,
        },
        link_type_name: "test-name",
        from_referenceable: from.to_owned(),
        to_referenceable: Some(to.to_owned()),
        delete,
        bidirectional,
    };
    h.render_template(&common_template_content, &data).unwrap()
}

fn test_header(from_referenceable: &Referenceable, to_referenceable: &Referenceable) -> String {
    let from = from_referenceable.to_string(&Cardinality::Single);
    let to = to_referenceable.to_string(&Cardinality::Single);
    let to_plural = pluralize(&to, 2, false);

    format!(
        r#"use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;
use test-zome::{from}_to_{to_plural}::*;

mod common;
use common::*;

#[tokio::test(flavor = "multi_thread")]
async fn link_a_{from}_to_a_{to}() {{
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/test-dna.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("test-zome");
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("test-zome");
"#
    )
}

fn test_body(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    deletable: bool,
    bidirectional: bool,
) -> String {
    let from = from_referenceable.to_string(&Cardinality::Single);
    let to = to_referenceable.to_string(&Cardinality::Single);
    let from_plural = pluralize(&from, 2, false);
    let to_plural = pluralize(&to, 2, false);
    let from_pascal_case = from.to_case(Case::Pascal);
    let to_pascal_case = to.to_case(Case::Pascal);
    let from_singular_arg = from_referenceable.field_name(&Cardinality::Single);
    let to_singular_arg = to_referenceable.field_name(&Cardinality::Single);

    let mut test_body = format!(
        r#"
    // Bob gets the links, should be empty
    let links_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{to_plural}_for_{from}",
            base_address.clone()
        )
        .await;
    assert!(links_output.is_empty());

    // Alice creates a link from {from_pascal_case} to {to_pascal_case}
    let _: () = alice_conductor
        .call(
            &alice_zome,
            "add_{to}_for_{from}",
            Add{to_pascal_case}For{from_pascal_case}Input {{
                base_{from_singular_arg}: base_address.clone(),
                target_{to_singular_arg}: target_address.clone(),
            }},
        )
        .await;

    await_consistency(&cells).await.unwrap();

    // Bob gets the links again
    let links_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{to_plural}_for_{from}",
            base_address.clone()
        )
        .await;
    assert_eq!(links_output.len(), 1);"#
    );

    if !matches!(to_referenceable, Referenceable::Agent { .. }) {
        test_body.push_str(
            r#"
    assert_eq!(
        links_output[0].target,
        target_address.clone().into()
    );"#,
        );
    }

    if bidirectional {
        test_body.push_str(&format!(
            r#"

    // Bob gets the links in the inverse direction
    let links_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{from_plural}_for_{to}",
            target_address.clone()
        )
        .await;
    assert_eq!(links_output.len(), 1);"#
        ));

        if !matches!(from_referenceable, Referenceable::Agent { .. }) {
            test_body.push_str(
                r#"
    assert_eq!(
        links_output[0].target,
        base_address.clone().into()
    );"#,
            );
        }
    }

    if deletable {
        test_body.push_str(&format!(
            r#"

    // Alice deletes the link
    let _: () = alice_conductor
        .call(
            &alice_zome,
            "delete_{to}_for_{from}",
            Remove{to_pascal_case}For{from_pascal_case}Input {{
                base_{from_singular_arg}: base_address.clone(),
                target_{to_singular_arg}: target_address.clone(),
            }},
        )
        .await;

    await_consistency(&cells).await.unwrap();

    // Bob gets the links again
    let links_output: Vec<Link> = bob_conductor
        .call(   
            &bob_zome,
            "get_{to_plural}_for_{from}",
            base_address.clone()
        )
        .await;
    assert!(links_output.is_empty());

    // Bob gets the deleted links
    let deleted_links_output: Vec<(SignedActionHashed, Vec<SignedActionHashed>)> = bob_conductor
        .call(
            &bob_zome,
            "get_deleted_{to_plural}_for_{from}",
            base_address.clone(),
        )
        .await;
    assert_eq!(deleted_links_output.len(), 1);"#
        ));

        if bidirectional {
            test_body.push_str(&format!(
                r#"

    // Bob gets the links in the inverse direction
    let links_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{from_plural}_for_{to}",
            target_address.clone()
        )
        .await;
    assert!(links_output.is_empty());

    // Bob gets the deleted links in the inverse direction
    let deleted_links_output: Vec<(SignedActionHashed, Vec<SignedActionHashed>)> = bob_conductor
        .call(
            &bob_zome,
            "get_deleted_{from_plural}_for_{to}",
            target_address.clone(),
        )
        .await;
    assert_eq!(deleted_links_output.len(), 1);"#,
            ));
        }
    }

    test_body.push_str(
        r#"
}
"#,
    );
    test_body
}

fn render_and_assert_eq(
    from: &Referenceable,
    to: &Referenceable,
    expected_addresses: &str,
    delete: bool,
    bidirectional: bool,
) {
    let rendered_test = render_template(&from, &to, delete, bidirectional);

    let mut expected_test = test_header(&from, &to);
    expected_test.push_str(expected_addresses);
    expected_test.push_str(&test_body(&from, &to, delete, bidirectional));

    pretty_assertions::assert_str_eq!(rendered_test, expected_test);
}

#[test]
fn agent_to_agent() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn agent_to_agent_deletable() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn agent_to_agent_bidirectional() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn agent_to_agent_deletable_bidirectional() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn entry_type_to_entry_type() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn entry_type_to_entry_type_deletable() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn entry_type_to_entry_type_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn entry_type_to_entry_type_deletable_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn agent_to_entry_type() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn agent_to_entry_type_deletable() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn agent_to_entry_type_bidirectional() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn agent_to_entry_type_deletable_bidirectional() {
    let from = Referenceable::Agent {
        role: "frommer".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = alice_zome.cell_id().agent_pubkey().clone();
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn entry_type_to_agent() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn entry_type_to_agent_deletable() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn entry_type_to_agent_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn entry_type_to_agent_deletable_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::Agent {
        role: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = alice_zome.cell_id().agent_pubkey().clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn entry_type_to_external_hash() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn entry_type_to_external_hash_deletable() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn entry_type_to_external_hash_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn entry_type_to_external_hash_deletable_bidirectional() {
    let from = Referenceable::EntryType(EntryTypeReference {
        entry_type: "frommer".to_string(),
        reference_entry_hash: false,
    });
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_record = create_frommer(&alice_conductor, &alice_zome).await;
    let base_address = base_record.signed_action.hashed.hash.clone();
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn external_hash_to_entry_type() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn external_hash_to_entry_type_deletable() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn external_hash_to_entry_type_bidirectional() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn external_hash_to_entry_type_deletable_bidirectional() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::EntryType(EntryTypeReference {
        entry_type: "toer".to_string(),
        reference_entry_hash: false,
    });
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_record = create_toer(&alice_conductor, &alice_zome).await;
    let target_address = target_record.signed_action.hashed.hash.clone();
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}

#[test]
fn external_hash_to_external_hash() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, false);
}

#[test]
fn external_hash_to_external_hash_deletable() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, false);
}

#[test]
fn external_hash_to_external_hash_bidirectional() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, false, true);
}

#[test]
fn external_hash_to_external_hash_deletable_bidirectional() {
    let from = Referenceable::ExternalHash {
        name: "from".to_string(),
    };
    let to = Referenceable::ExternalHash {
        name: "toer".to_string(),
    };
    let expected_addresses = r#"
    let base_address = ExternalHash::from_raw_36(vec![0; 36]);
    let target_address = ExternalHash::from_raw_36(vec![0; 36]);
"#;

    render_and_assert_eq(&from, &to, expected_addresses, true, true);
}
