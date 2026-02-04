use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::{
        collection::CollectionType,
        entry_type::definitions::{EntryTypeReference, Referenceable},
    },
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldCollectionData {
    pub app_name: String,
    pub dna_role_name: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub collection_type: CollectionType,
    pub collection_name: String,
    pub referenceable: Referenceable,
    pub deletable: bool,
}

// TODO: group some params into a new-type or prefer builder pattern
#[allow(unknown_lints, clippy::too_many_arguments, clippy::manual_inspect)]
pub fn scaffold_collection_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &str,
    dna_role_name: &str,
    coordinator_zome_manifest: &ZomeManifest,
    collection_type: &CollectionType,
    collection_name: &str,
    entry_type_reference: &EntryTypeReference,
    deletable: bool,
    no_ui: bool,
    no_spec: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldCollectionData {
        app_name: app_name.to_owned(),
        dna_role_name: dna_role_name.to_owned(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        collection_name: collection_name.to_owned(),
        collection_type: *collection_type,
        referenceable: Referenceable::EntryType(entry_type_reference.clone()),
        deletable,
    };

    let h = build_handlebars(template_file_tree)?;

    let field_types_path = PathBuf::from("collection");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        let mut web_app_template = web_app_template.clone();
        if no_ui {
            web_app_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "ui");
                v
            });
        }
        if no_spec {
            web_app_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "tests" && k != "dnas");
                v
            });
        }
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            &web_app_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        template_file_tree,
        &PathBuf::from("collection.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scaffold::web_app::template_type::TemplateType;
    use build_fs_tree::{dir, file};
    use std::path::PathBuf;

    struct CollectionTestParams {
        reference_entry_hash: bool,
        collection_type: CollectionType,
        deletable: bool,
    }

    fn render_collection_test(params: &CollectionTestParams) -> String {
        let app_file_tree: FileTree = dir! {
            "dnas" => dir! {
                "test_dna" => dir! {
                    "zomes" => dir! {
                        "coordinator" => dir! {
                            "test_zome" => dir! {
                                "src" => dir! {
                                    "lib.rs" => file!("")
                                }
                            }
                        }
                    }
                }
            }
        };
        let template_file_tree = TemplateType::Vanilla.file_tree().unwrap();
        let coordinator_zome_manifest = ZomeManifest {
            name: "test_zome".into(),
            hash: None,
            path: "test.wasm".into(),
            dependencies: None,
        };
        let entry_type_reference = EntryTypeReference {
            entry_type: "TestPost".to_string(),
            reference_entry_hash: params.reference_entry_hash,
        };

        let result = scaffold_collection_templates(
            app_file_tree,
            &template_file_tree,
            "test_app",
            "test_dna",
            &coordinator_zome_manifest,
            &params.collection_type,
            "all_posts",
            &entry_type_reference,
            params.deletable,
            false,
            false,
        )
        .unwrap();

        let coordinator_zome_path = PathBuf::from("dnas/test_dna/zomes/coordinator/test_zome");
        file_content(
            &result.file_tree,
            &coordinator_zome_path.join("tests/all-posts.rs"),
        )
        .unwrap()
    }

    fn expected_collection_test(params: &CollectionTestParams) -> String {
        let reference = if params.reference_entry_hash {
            "create_record.signed_action.hashed.content.entry_hash().unwrap().clone().into()"
        } else {
            "create_record.signed_action.hashed.hash.clone().into()"
        };

        let posts_by = if matches!(params.collection_type, CollectionType::ByAuthor) {
            "alice_zome.cell_id().agent_pubkey().clone()"
        } else {
            "()"
        };

        let delete_section = if params.deletable && !params.reference_entry_hash {
            format!(
                r#"

    // Alice deletes the TestPost
    let _delete_action_hash: ActionHash = alice_conductor
        .call(
            &alice_zome,
            "delete_test_post",
            create_record.signed_action.hashed.hash.clone(),
        )
        .await;

    // Wait for the entry deletion to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets all posts again
    let collection_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_all_posts",
            {posts_by},
        )
        .await;
    assert_eq!(collection_output.len(), 0);"#
            )
        } else {
            "".to_string()
        };

        format!(
            r#"use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;

mod common;
use common::*;

#[tokio::test(flavor = "multi_thread")]
async fn create_a_test_post_and_get_all_posts() {{
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::standard(2).await;
    let dna_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/test_dna.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("test_zome");
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("test_zome");

    // Bob gets all posts
    let collection_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_all_posts",
            {posts_by},
        )
        .await;
    assert_eq!(collection_output.len(), 0);

    // Alice creates a TestPost
    let create_record: Record = create_test_post(&alice_conductor, &alice_zome).await;

    // Wait for the created entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets all posts again
    let collection_output: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_all_posts",
            {posts_by},
        )
        .await;
    assert_eq!(collection_output.len(), 1);
    assert_eq!(
        collection_output[0].target,
        {reference}
    );{delete_section}
}}
"#
        )
    }

    #[test]
    fn scaffold_collection_global() {
        let params = CollectionTestParams {
            reference_entry_hash: false,
            collection_type: CollectionType::Global,
            deletable: false,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_reference_entry_hash_global() {
        let params = CollectionTestParams {
            reference_entry_hash: true,
            collection_type: CollectionType::Global,
            deletable: false,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_global_deletable() {
        let params = CollectionTestParams {
            reference_entry_hash: false,
            collection_type: CollectionType::Global,
            deletable: true,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_reference_entry_hash_global_deletable() {
        let params = CollectionTestParams {
            reference_entry_hash: true,
            collection_type: CollectionType::Global,
            deletable: true,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_by_author() {
        let params = CollectionTestParams {
            reference_entry_hash: false,
            collection_type: CollectionType::ByAuthor,
            deletable: false,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_reference_entry_hash_by_author() {
        let params = CollectionTestParams {
            reference_entry_hash: true,
            collection_type: CollectionType::ByAuthor,
            deletable: false,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_reference_entry_hash_by_author_deletable() {
        let params = CollectionTestParams {
            reference_entry_hash: true,
            collection_type: CollectionType::ByAuthor,
            deletable: true,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }

    #[test]
    fn scaffold_collection_by_author_deletable() {
        let params = CollectionTestParams {
            reference_entry_hash: false,
            collection_type: CollectionType::ByAuthor,
            deletable: true,
        };
        pretty_assertions::assert_str_eq!(
            render_collection_test(&params),
            expected_collection_test(&params)
        );
    }
}
