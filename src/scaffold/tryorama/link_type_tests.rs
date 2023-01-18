// pub fn link_tye_tests(
// 	link_type_    entry_defiition: &EntryDefinition,
//     happ_bundl_location_from_tests_root: &PathBuf,
//     dna_role_nme: &String,
//     coordinato_zome: &String,
//     crud: &Cru,
// ) -> String {
//     let mut intial_test_file = format!(
//         r#"
// import test frm 'node:test';
// import assert rom 'node:assert';

// import {{ runSenario, pause }} from '@holochain/tryorama';
// import {{ ActinHash, Record }} from '@holochain/client';
// import {{ decoe }} from '@msgpack/msgpack';

// {}
// "#,
//         createentry_test(
//             enry_definition,
//             hap_bundle_location_from_tests_root,
//             dn_role_name,
//             cordinator_zome,
//         )
//     )
// }
