use super::super::*;
use crate::scaffold::entry_type::definitions::{
    Cardinality, EntryTypeReference, FieldDefinition, Referenceable,
};
use crate::scaffold::web_app::template_type::TemplateType;
use crate::{file_tree::file_exists, scaffold::entry_type::definitions::FieldType};
use build_fs_tree::{dir, file, FileSystemTree};

// Expected string helpers

fn expected_header(entry_type_name: &str) -> String {
    format!(
        r#"use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;
use test_zome::{entry_type_name}::*;
use test_zome_integrity::*;

mod common;
use common::*;
"#
    )
}

fn expected_create_test(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
) -> String {
    format!(
        r#"
#[tokio::test(flavor = "multi_thread")]
async fn create_{entry_type_name_snake_case}() {{
    // Create a conductor with the standard config
    let mut conductor = SweetConductor::standard().await;
    let dna_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/test_dna.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let app = conductor.setup_app("test-app", &[dna_file]).await.unwrap();
    let zome = app.cells()[0].zome("test_zome");

    let {entry_type_name_snake_case} = sample_{entry_type_name_snake_case}(&conductor, &zome).await;

    // Agent creates a {entry_type_name_pascal_case}
    let _: Record = conductor.call(&zome, "create_{entry_type_name_snake_case}", {entry_type_name_snake_case}.clone()).await;
}}
"#
    )
}

fn expected_create_and_read_test(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    get_fn_name: &str,
    hash_access: &str,
    linked_from: Option<&str>,
) -> String {
    let linked_from_section = linked_from
        .map(|field_access| {
            format!(
                r#"

    // Bob gets the {entry_type_name_pascal_case}s for the new TestPost
    let links_to_test_posts: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{entry_type_name_snake_case}s_for_test_post",
            {entry_type_name_pascal_case}::try_from({entry_type_name_snake_case}.clone()).unwrap().{field_access},
        )
        .await;
    assert_eq!(links_to_test_posts.len(), 1);
    assert_eq!(
        links_to_test_posts[0].target,
        record.signed_action.hashed.{hash_access}.into()
    );"#
            )
        })
        .unwrap_or_default();

    format!(
        r#"
#[tokio::test(flavor = "multi_thread")]
async fn create_and_read_{entry_type_name_snake_case}() {{
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

    let {entry_type_name_snake_case} = sample_{entry_type_name_snake_case}(&alice_conductor, &alice_zome).await;

    // Alice creates a {entry_type_name_pascal_case}
    let record: Record = alice_conductor
        .call(&alice_zome, "create_{entry_type_name_snake_case}", {entry_type_name_snake_case}.clone())
        .await;

    // Wait for the created entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets the created {entry_type_name_pascal_case}
    let propagated_record: Record = bob_conductor
        .call(
            &bob_zome,
            "{get_fn_name}",
            record.signed_action.hashed.{hash_access},
        )
        .await;
    assert_eq!(record, propagated_record);{linked_from_section}
}}
"#
    )
}

fn expected_update_test(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    link_from_original_to_each_update: bool,
) -> String {
    let maybe_original_hash = if link_from_original_to_each_update {
        format!(
            r#"
        original_{entry_type_name_snake_case}_hash: original_action_hash.clone(),"#
        )
    } else {
        "".to_string()
    };

    format!(
        r#"
#[tokio::test(flavor = "multi_thread")]
async fn create_and_update_{entry_type_name_snake_case}() {{
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

    let {entry_type_name_snake_case} = sample_{entry_type_name_snake_case}(&alice_conductor, &alice_zome).await;

    // Alice creates a {entry_type_name_pascal_case}
    let record: Record = alice_conductor
        .call(&alice_zome, "create_{entry_type_name_snake_case}", {entry_type_name_snake_case}.clone())
        .await;

    let original_action_hash = record.signed_action.hashed.hash.clone();

    // Alice updates the {entry_type_name_pascal_case}
    let content_update = sample_{entry_type_name_snake_case}(&alice_conductor, &alice_zome).await;
    let update_input = Update{entry_type_name_pascal_case}Input {{{maybe_original_hash}
        previous_{entry_type_name_snake_case}_hash: original_action_hash.clone(),
        updated_{entry_type_name_snake_case}: content_update.clone(),
    }};
    let updated_record: Record = alice_conductor
        .call(&alice_zome, "update_{entry_type_name_snake_case}", update_input)
        .await;

    // Wait for the updated entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets the updated {entry_type_name_pascal_case}
    let read_updated_record_1: Record = bob_conductor
        .call(
            &bob_zome,
            "get_latest_{entry_type_name_snake_case}",
            updated_record.signed_action.hashed.hash.clone(),
        )
        .await;
    assert_eq!(updated_record, read_updated_record_1);

    // Alice updates the {entry_type_name_pascal_case} again
    let content_update = sample_{entry_type_name_snake_case}(&alice_conductor, &alice_zome).await;
    let update_input = Update{entry_type_name_pascal_case}Input {{{maybe_original_hash}
        previous_{entry_type_name_snake_case}_hash: updated_record.signed_action.hashed.hash.clone(),
        updated_{entry_type_name_snake_case}: content_update.clone(),
    }};
    let updated_record: Record = alice_conductor
        .call(&alice_zome, "update_{entry_type_name_snake_case}", update_input)
        .await;

    // Wait for the updated entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets the updated {entry_type_name_pascal_case}
    let read_updated_record_2: Record = bob_conductor
        .call(
            &bob_zome,
            "get_latest_{entry_type_name_snake_case}",
            updated_record.signed_action.hashed.hash.clone(),
        )
        .await;
    let RecordEntry::Present(entry) = read_updated_record_2.entry else {{
        panic!(
            "Expected Present entry, got {{:?}}",
            read_updated_record_2.entry
        );
    }};
    assert_eq!({entry_type_name_pascal_case}::try_from(entry.clone()).unwrap(), content_update);

    // Bob gets all the revisions for {entry_type_name_pascal_case}
    let revisions: Vec<Record> = bob_conductor
        .call(&bob_zome, "get_all_revisions_for_{entry_type_name_snake_case}", original_action_hash)
        .await;
    assert_eq!(revisions.len(), 3);
    let RecordEntry::Present(ref entry) = revisions[2].entry else {{
        panic!("Expected Present entry, got {{:?}}", revisions[2].entry);
    }};
    assert_eq!({entry_type_name_pascal_case}::try_from(entry).unwrap(), content_update);
}}
"#
    )
}

fn expected_delete_test(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    hash_access: &str,
    linked_from: Option<&str>,
) -> String {
    let linked_from_section_before_delete = linked_from
        .map(|field_access| {
            format!(
                r#"

    // Bob gets the {entry_type_name_pascal_case}s for the new TestPost
    let links_to_test_posts: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{entry_type_name_snake_case}s_for_test_post",
            {entry_type_name_pascal_case}::try_from({entry_type_name_snake_case}.clone()).unwrap().{field_access},
        )
        .await;
    assert_eq!(links_to_test_posts.len(), 1);
    assert_eq!(
        links_to_test_posts[0].target,
        record.signed_action.hashed.{hash_access}.into()
    );"#
            )
        })
        .unwrap_or_default();

    let linked_from_section_after_delete = linked_from
        .map(|field_access| {
            format!(
                r#"

    // Bob gets the {entry_type_name_pascal_case}s for the TestPost again
    let links_to_test_posts: Vec<Link> = bob_conductor
        .call(
            &bob_zome,
            "get_{entry_type_name_snake_case}s_for_test_post",
            {entry_type_name_pascal_case}::try_from({entry_type_name_snake_case}.clone()).unwrap().{field_access},
        )
        .await;
    assert_eq!(links_to_test_posts.len(), 0);

    // Bob gets the deleted {entry_type_name_pascal_case} for the TestPosts
    let deleted_links_to_test_posts: Vec<(SignedActionHashed, Vec<SignedActionHashed>)> = bob_conductor
        .call(
            &bob_zome,
            "get_deleted_{entry_type_name_snake_case}s_for_test_post",
            {entry_type_name_pascal_case}::try_from({entry_type_name_snake_case}.clone()).unwrap().{field_access},
        )
        .await;
    assert_eq!(deleted_links_to_test_posts.len(), 1);"#
            )
        })
        .unwrap_or_default();

    format!(
        r#"
#[tokio::test(flavor = "multi_thread")]
async fn create_and_delete_{entry_type_name_snake_case}() {{
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

    let {entry_type_name_snake_case} = sample_{entry_type_name_snake_case}(&alice_conductor, &alice_zome).await;

    // Alice creates a {entry_type_name_pascal_case}
    let record: Record = alice_conductor
        .call(&alice_zome, "create_{entry_type_name_snake_case}", {entry_type_name_snake_case}.clone())
        .await;

    // Wait for the created entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();{linked_from_section_before_delete}

    // Alice deletes the {entry_type_name_pascal_case}
    let _delete_action_hash: ActionHash = alice_conductor
        .call(
            &alice_zome,
            "delete_{entry_type_name_snake_case}",
            record.signed_action.hashed.hash.clone(),
        )
        .await;

    // Wait for the entry deletion to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets the oldest delete for the {entry_type_name_pascal_case}
    let oldest_delete_for_{entry_type_name_snake_case}: Option<SignedActionHashed> = bob_conductor
        .call(
            &bob_zome,
            "get_oldest_delete_for_{entry_type_name_snake_case}",
            record.signed_action.hashed.hash.clone(),
        )
        .await;
    assert!(oldest_delete_for_{entry_type_name_snake_case}.is_some());

    // Bob gets the deletions for the {entry_type_name_pascal_case}
    let deletes_for_{entry_type_name_snake_case}: Option<Vec<SignedActionHashed>> = bob_conductor
        .call(
            &bob_zome,
            "get_all_deletes_for_{entry_type_name_snake_case}",
            record.signed_action.hashed.hash.clone(),
        )
        .await;
    assert_eq!(deletes_for_{entry_type_name_snake_case}.unwrap().len(), 1);{linked_from_section_after_delete}
}}
"#
    )
}

/// Build expected test file with create-only tests
fn expected_rendered_create_and_read(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    get_fn_name: &str,
    hash_access: &str,
    linked_from: Option<&str>,
) -> String {
    format!(
        "{}{}{}",
        expected_header(entry_type_name_snake_case),
        expected_create_test(entry_type_name_snake_case, entry_type_name_pascal_case),
        expected_create_and_read_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            get_fn_name,
            hash_access,
            linked_from,
        )
    )
}

/// Build expected test file with create + update tests
fn expected_rendered_create_and_update(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    link_from_original_to_each_update: bool,
) -> String {
    format!(
        "{}{}{}{}",
        expected_header(entry_type_name_snake_case),
        expected_create_test(entry_type_name_snake_case, entry_type_name_pascal_case),
        expected_create_and_read_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            &format!("get_original_{entry_type_name_snake_case}"),
            "hash.clone()",
            None,
        ),
        expected_update_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            link_from_original_to_each_update
        )
    )
}

/// Build expected test file with create + delete tests
fn expected_rendered_create_and_delete(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    hash_access: &str,
    linked_from: Option<&str>,
) -> String {
    format!(
        "{}{}{}{}",
        expected_header(entry_type_name_snake_case),
        expected_create_test(entry_type_name_snake_case, entry_type_name_pascal_case),
        expected_create_and_read_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            &format!("get_{entry_type_name_snake_case}"),
            "hash.clone()",
            linked_from,
        ),
        expected_delete_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            hash_access,
            linked_from
        )
    )
}

/// Build expected test file with create + update + delete tests
fn expected_rendered_create_and_update_and_delete(
    entry_type_name_snake_case: &str,
    entry_type_name_pascal_case: &str,
    link_from_original_to_each_update: bool,
    hash_access: &str,
    linked_from: Option<&str>,
) -> String {
    format!(
        "{}{}{}{}{}",
        expected_header(entry_type_name_snake_case),
        expected_create_test(entry_type_name_snake_case, entry_type_name_pascal_case),
        expected_create_and_read_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            &format!("get_original_{entry_type_name_snake_case}"),
            "hash.clone()",
            None,
        ),
        expected_update_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            link_from_original_to_each_update
        ),
        expected_delete_test(
            entry_type_name_snake_case,
            entry_type_name_pascal_case,
            hash_access,
            linked_from
        )
    )
}

// Test setup

struct TestCase {
    app_file_tree: FileTree,
    template_file_tree: FileSystemTree<OsString, String>,
    integrity_zome_manifest: ZomeManifest,
    coordinator_zome_manifest: ZomeManifest,
    coordinator_zome_path: PathBuf,
    entry_type: EntryDefinition,
}

fn scaffold_test_entry_type() -> TestCase {
    // Create an app file tree with the coordinator zome directory already present
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
    let integrity_zome_manifest = ZomeManifest {
        name: "test_zome_integrity".into(),
        hash: None,
        path: "test_integrity.wasm".into(),
        dependencies: None,
    };
    let coordinator_zome_manifest = ZomeManifest {
        name: "test_zome".into(),
        hash: None,
        path: "test.wasm".into(),
        dependencies: None,
    };
    let coordinator_zome_path = PathBuf::from("dnas/test_dna/zomes/coordinator/test_zome");
    let entry_type = EntryDefinition {
        name: "TestPost".to_string(),
        fields: vec![FieldDefinition {
            field_name: "title".to_string(),
            field_type: FieldType::String,
            widget: Some("TextField".to_string()),
            cardinality: Cardinality::Single,
            linked_from: None,
        }],
        reference_entry_hash: false,
    };
    TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    }
}

#[test]
fn scaffold_entry_type_creates_test_files() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud::default();

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    assert!(file_exists(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs")
    ));
    assert!(file_exists(
        &result.file_tree,
        &coordinator_zome_path.join("tests/common.rs")
    ));
}

#[test]
fn scaffold_entry_type_create() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: false,
        delete: false,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_read(
            "test_post",
            &entry_type.name,
            "get_test_post",
            "hash.clone()",
            None,
        )
    );
}

#[test]
fn scaffold_entry_type_create_with_entry_hash_reference() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        mut entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: false,
        delete: false,
    };
    entry_type.reference_entry_hash = true;

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_read(
            "test_post",
            &entry_type.name,
            "get_test_post",
            "content.entry_hash().unwrap().clone()",
            None,
        )
    );
}

#[test]
fn scaffold_entry_type_create_update() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: true,
        delete: false,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_update("test_post", &entry_type.name, false)
    );
}

#[test]
fn scaffold_entry_type_create_update_link_from_original_to_each_update() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: true,
        delete: false,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        true,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_update("test_post", &entry_type.name, true)
    );
}

#[test]
fn scaffold_entry_type_create_update_delete() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: true,
        delete: true,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_update_and_delete(
            "test_post",
            &entry_type.name,
            false,
            "hash.clone()",
            None
        )
    );
}

#[test]
fn scaffold_entry_type_create_delete() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        entry_type,
    } = scaffold_test_entry_type();
    let crud = Crud {
        update: false,
        delete: true,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-post.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_delete("test_post", &entry_type.name, "hash.clone()", None)
    );
}

#[test]
fn scaffold_entry_type_create_linked_from() {
    // Scaffold a test entry with a field that links to another entry.
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        ..
    } = scaffold_test_entry_type();
    let entry_type = EntryDefinition {
        name: "TestComment".to_string(),
        fields: vec![
            FieldDefinition {
                field_name: "test_comment".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: None,
            },
            FieldDefinition {
                field_name: "test_post_hash".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                    entry_type: "test_post".to_string(),
                    reference_entry_hash: false,
                })),
            },
        ],
        reference_entry_hash: false,
    };
    let crud = Crud {
        update: false,
        delete: false,
    };
    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-comment.rs"),
    )
    .unwrap();
    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_read(
            "test_comment",
            "TestComment",
            "get_test_comment",
            "hash.clone()",
            Some("test_post_hash")
        )
    );
}

#[test]
fn scaffold_entry_type_create_linked_from_with_entry_hash_reference() {
    // Test the linked_from path with reference_entry_hash: true
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        ..
    } = scaffold_test_entry_type();
    let entry_type = EntryDefinition {
        name: "TestComment".to_string(),
        fields: vec![
            FieldDefinition {
                field_name: "test_comment".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: None,
            },
            FieldDefinition {
                field_name: "test_post_hash".to_string(),
                field_type: FieldType::EntryHash,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                    entry_type: "test_post".to_string(),
                    reference_entry_hash: false,
                })),
            },
        ],
        reference_entry_hash: true,
    };
    let crud = Crud {
        update: false,
        delete: false,
    };
    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-comment.rs"),
    )
    .unwrap();
    // Verify the entry_hash path is used in the linked_from assertion
    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_read(
            "test_comment",
            &entry_type.name,
            "get_test_comment",
            "content.entry_hash().unwrap().clone()",
            Some("test_post_hash"),
        )
    );
}

#[test]
fn scaffold_entry_type_create_linked_from_vector() {
    // Scaffold a test entry with a field that links to another entry.
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        ..
    } = scaffold_test_entry_type();
    let entry_type = EntryDefinition {
        name: "TestComment".to_string(),
        fields: vec![
            FieldDefinition {
                field_name: "test_comment".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: None,
            },
            FieldDefinition {
                field_name: "test_post_hash".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Vector,
                linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                    entry_type: "test_post".to_string(),
                    reference_entry_hash: false,
                })),
            },
        ],
        reference_entry_hash: false,
    };
    let crud = Crud {
        update: false,
        delete: false,
    };
    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-comment.rs"),
    )
    .unwrap();
    // Verify the vector field uses [0] index access
    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_read(
            "test_comment",
            &entry_type.name,
            "get_test_comment",
            "hash.clone()",
            Some("test_post_hash[0]"),
        ),
        "Expected vector field to use [0] index access. Got:\n{scaffolded_test_file}"
    );
}

#[test]
fn scaffold_entry_type_create_delete_linked_from() {
    let TestCase {
        app_file_tree,
        template_file_tree,
        integrity_zome_manifest,
        coordinator_zome_manifest,
        coordinator_zome_path,
        ..
    } = scaffold_test_entry_type();
    let entry_type = EntryDefinition {
        name: "TestComment".to_string(),
        fields: vec![
            FieldDefinition {
                field_name: "test_comment".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: None,
            },
            FieldDefinition {
                field_name: "test_post_hash".to_string(),
                field_type: FieldType::String,
                widget: None,
                cardinality: Cardinality::Single,
                linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                    entry_type: "test_post".to_string(),
                    reference_entry_hash: false,
                })),
            },
        ],
        reference_entry_hash: false,
    };
    let crud = Crud {
        update: false,
        delete: true,
    };

    let result = scaffold_entry_type_templates(
        app_file_tree,
        &template_file_tree,
        "test_app",
        "test_dna",
        &integrity_zome_manifest,
        &coordinator_zome_manifest,
        &entry_type,
        "",
        &crud,
        false,
        false,
        false,
    )
    .unwrap();

    let scaffolded_test_file = file_content(
        &result.file_tree,
        &coordinator_zome_path.join("tests/test-comment.rs"),
    )
    .unwrap();

    pretty_assertions::assert_str_eq!(
        scaffolded_test_file,
        expected_rendered_create_and_delete(
            "test_comment",
            &entry_type.name,
            "hash.clone()",
            Some("test_post_hash")
        )
    );
}
