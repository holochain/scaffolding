use std::path::PathBuf;

use convert_case::{Case, Casing};

use crate::scaffold::entry_type::{
    crud::Crud,
    definitions::{EntryDefinition, EntryTypeReference, Referenceable},
};

use super::utils::common_tests_setup;

pub fn entry_crud_tests(
    entry_definition: &EntryDefinition,
    app_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    crud: &Crud,
    link_original_to_each_update: bool,
) -> String {
    let mut initial_test_file = format!(
        r#"
import test from 'node:test';
import assert from 'node:assert';

import {{ runScenario, pause, CallableCell }} from '@holochain/tryorama';
import {{ NewEntryAction, ActionHash, Record, AppBundleSource }} from '@holochain/client';
import {{ decode }} from '@msgpack/msgpack';

{}
"#,
        create_entry_test(
            entry_definition,
            app_bundle_location_from_tests_root,
            coordinator_zome,
        )
    );

    initial_test_file.push_str(
        read_entry_test(
            entry_definition,
            app_bundle_location_from_tests_root,
            coordinator_zome,
        )
        .as_str(),
    );

    if crud.update {
        initial_test_file.push_str(
            update_entry_test(
                entry_definition,
                app_bundle_location_from_tests_root,
                coordinator_zome,
                link_original_to_each_update,
            )
            .as_str(),
        );
    }

    if crud.delete {
        initial_test_file.push_str(
            delete_entry_test(
                entry_definition,
                app_bundle_location_from_tests_root,
                coordinator_zome,
            )
            .as_str(),
        );
    }

    initial_test_file
}

pub fn create_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
) -> String {
    let pascal_entry_def_name = entry_definition.name.to_case(Case::Pascal);
    let snake_entry_def_name = entry_definition.name.to_case(Case::Snake);

    let deps: Vec<EntryTypeReference> = entry_definition
        .fields
        .iter()
        .filter_map(|f| f.linked_from.clone())
        .filter_map(|r| match r {
            Referenceable::EntryType(entry_type_referencable) => Some(entry_type_referencable),
            _ => None,
        })
        .collect();
    let imports_create_from_deps = deps
        .iter()
        .map(|dependant_referenceable| {
            format!(
                "import {{ create{} }} from \"./{}.test.js\";",
                dependant_referenceable.entry_type.to_case(Case::Pascal),
                dependant_referenceable.entry_type.to_case(Case::Snake)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"{imports_create_from_deps}
async function sample{pascal_entry_def_name}(cell: CallableCell, partial{pascal_entry_def_name} = {{}}) {{
    return {{
        ...{},
        ...partial{pascal_entry_def_name}
    }};
}}

export async function create{pascal_entry_def_name}(cell: CallableCell, {snake_entry_def_name} = undefined): Promise<Record> {{
    return cell.callZome({{
      zome_name: "{coordinator_zome}",
      fn_name: "create_{snake_entry_def_name}",
      payload: {snake_entry_def_name} || await sample{pascal_entry_def_name}(cell),
    }});
}}

test('create {}', {{ concurrency: 1 }}, async t => {{
  await runScenario(async scenario => {{
{}

    // Alice creates a {pascal_entry_def_name}
    const record: Record = await create{pascal_entry_def_name}(alice.cells[0]);
    assert.ok(record);
  }});
}});
"#,
        entry_definition.js_sample_object(),
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
    )
}

pub fn hash_from_record(record_ident: String, reference_entry_hash: bool) -> String {
    match reference_entry_hash {
        true => {
            format!("({record_ident}.signed_action.hashed.content as NewEntryAction).entry_hash")
        }
        false => format!("{record_ident}.signed_action.hashed.hash"),
    }
}

pub fn read_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
) -> String {
    let pascal_entry_def_name = entry_definition.name.to_case(Case::Pascal);
    let hash_to_get_from = hash_from_record(
        String::from("record"),
        entry_definition.reference_entry_hash,
    );

    format!(
        r#"
test('create and read {}', {{ concurrency: 1 }}, async t => {{
  await runScenario(async scenario => {{
{}

    const sample = await sample{pascal_entry_def_name}(alice.cells[0]);

    // Alice creates a {pascal_entry_def_name}
    const record: Record = await create{pascal_entry_def_name}(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(800);

    // Bob gets the created {}
    const createReadOutput: Record = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: {hash_to_get_from},
    }});
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  }});
}});
"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
        entry_definition.name,
        coordinator_zome,
        entry_definition.name
    )
}

pub fn update_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    link_original_to_each_update: bool,
) -> String {
    let pascal_entry_def_name = entry_definition.name.to_case(Case::Pascal);
    let snake_entry_def_name = entry_definition.name.to_case(Case::Snake);
    let read_after_update = |n: u32| {
        format!(
            r#"
    // Wait for the updated entry to be propagated to the other node.
    await pause(800);
        
    // Bob gets the updated {}
    const readUpdatedOutput{n}: Record = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: updatedRecord.signed_action.hashed.hash,
    }});
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput{n}.entry as any).Present.entry) as any);
"#,
            entry_definition.name.to_case(Case::Snake),
            coordinator_zome,
            entry_definition.name.to_case(Case::Snake)
        )
    };

    let original_action_hash_field = match link_original_to_each_update {
        true => format!(
            "
            original_{snake_entry_def_name}_hash: originalActionHash,",
        ),
        false => String::from(""),
    };

    format!(
        r#"
test('create and update {}', {{ concurrency: 1 }}, async t => {{
  await runScenario(async scenario => {{
{}

    // Alice creates a {pascal_entry_def_name}
    const record: Record = await create{pascal_entry_def_name}(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the {pascal_entry_def_name}
    let contentUpdate: any = await sample{pascal_entry_def_name}(alice.cells[0]);
    let updateInput = {{ {original_action_hash_field}
      previous_{snake_entry_def_name}_hash: originalActionHash,
      updated_{snake_entry_def_name}: contentUpdate,
    }};

    let updatedRecord: Record = await alice.cells[0].callZome({{
      zome_name: "{coordinator_zome}",
      fn_name: "update_{snake_entry_def_name}",
      payload: updateInput,
    }});
    assert.ok(updatedRecord);

{}

    // Alice updates the {pascal_entry_def_name} again
    contentUpdate = await sample{pascal_entry_def_name}(alice.cells[0]);
    updateInput = {{ {original_action_hash_field}
      previous_{snake_entry_def_name}_hash: updatedRecord.signed_action.hashed.hash,
      updated_{snake_entry_def_name}: contentUpdate,
    }};

    updatedRecord = await alice.cells[0].callZome({{
      zome_name: "{coordinator_zome}",
      fn_name: "update_{snake_entry_def_name}",
      payload: updateInput,
    }});
    assert.ok(updatedRecord);

{}
  }});
}});
"#,
        entry_definition.name.to_case(Case::Snake),
        common_tests_setup(happ_bundle_location_from_tests_root),
        read_after_update(0),
        read_after_update(1)
    )
}

pub fn delete_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
) -> String {
    let pascal_entry_def_name = entry_definition.name.to_case(Case::Pascal);

    let read_after_update = format!(
        r#"
    // Wait for the entry deletion to be propagated to the other node.
    await pause(800);
        
    // Bob tries to get the deleted {}
    const readDeletedOutput = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: record.signed_action.hashed.hash,
    }});
    assert.equal(readDeletedOutput, undefined);
"#,
        entry_definition.name, coordinator_zome, entry_definition.name
    );
    format!(
        r#"
test('create and delete {}', {{ concurrency: 1 }}, async t => {{
  await runScenario(async scenario => {{
{}

    // Alice creates a {pascal_entry_def_name}
    const record: Record = await create{pascal_entry_def_name}(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the {pascal_entry_def_name}
    const deleteActionHash = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "delete_{}",
      payload: record.signed_action.hashed.hash,
    }});
    assert.ok(deleteActionHash);

{}
  }});
}});
"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
        coordinator_zome,
        entry_definition.name,
        read_after_update
    )
}
