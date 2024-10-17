use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    error::ScaffoldResult,
    file_tree::{insert_file, map_file},
    scaffold::{
        dna::DnaFileTree,
        entry_type::definitions::{Cardinality, EntryTypeReference, Referenceable},
        zome::ZomeFileTree,
    },
    utils::unparse_pretty,
};

use super::link_type_name;

pub fn add_link_type_functions_to_coordinator(
    coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &str,
    link_type_name: &str,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
    bidirectional: bool,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = coordinator_zome_file_tree
        .dna_file_tree
        .dna_manifest_path
        .clone();
    let zome_manifest = coordinator_zome_file_tree.zome_manifest.clone();

    let snake_link_type_name = link_type_name.to_case(Case::Snake);
    let new_file_path = coordinator_zome_file_tree
        .zome_crate_path
        .join("src")
        .join(format!("{}.rs", &snake_link_type_name));
    let crate_src_path = coordinator_zome_file_tree.zome_crate_path.join("src");

    let lib_rs_path = crate_src_path.join("lib.rs");
    let mut file_tree = coordinator_zome_file_tree.dna_file_tree.file_tree();

    let link_type_handlers_file = match to_referenceable {
        Some(r) => normal_handlers(
            integrity_zome_name,
            from_referenceable,
            r,
            delete,
            bidirectional,
        ),
        None => metadata_handlers(integrity_zome_name, link_type_name, from_referenceable),
    };

    let file = unparse_pretty(&syn::parse_quote! { #link_type_handlers_file });

    // insert handlers file
    insert_file(&mut file_tree, &new_file_path, &file)?;

    // add newly created file to lib.rs file
    map_file(&mut file_tree, &lib_rs_path, |contents| {
        Ok(format!("pub mod {snake_link_type_name};\n{contents}",))
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}

fn normal_handlers(
    integrity_zome_name: &str,
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    delete: bool,
    bidirectional: bool,
) -> TokenStream {
    let inverse_get_handler = bidirectional
        .then(|| get_links_handler(to_referenceable, from_referenceable, delete))
        .unwrap_or_default();

    let delete_link_handler = delete
        .then(|| remove_link_handlers(from_referenceable, to_referenceable, bidirectional))
        .unwrap_or_default();

    let integrity_zome_name = format_ident!("{integrity_zome_name}");
    let add_links_handler = add_link_handler(from_referenceable, to_referenceable, bidirectional);
    let get_links_handler = get_links_handler(from_referenceable, to_referenceable, delete);

    quote! {
        use hdk::prelude::*;
        use #integrity_zome_name::*;

        #add_links_handler

        #get_links_handler

        #inverse_get_handler

        #delete_link_handler
    }
}

fn metadata_handlers(
    integrity_zome_name: &str,
    link_type_name: &str,
    from_referenceable: &Referenceable,
) -> TokenStream {
    let integrity_zome_name = format_ident!("{}", integrity_zome_name);
    let snake_from_arg = format_ident!(
        "{}",
        from_referenceable
            .field_name(&Cardinality::Single)
            .to_case(Case::Snake)
    );
    let from_field_type = format_ident!("{}", from_referenceable.field_type().to_string());
    let snake_from = format_ident!(
        "{}",
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake)
    );
    let pascal_from = format_ident!(
        "{}",
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Pascal)
    );
    let pascal_link_type_name = format_ident!("{}", link_type_name.to_case(Case::Pascal));
    let snake_link_type_name = format_ident!("{}", link_type_name.to_case(Case::Snake));
    let create_link_input_struct_name =
        format_ident!("Add{pascal_link_type_name}For{pascal_from}Input");
    let create_link_function_name = format_ident!("add_{snake_link_type_name}_for_{snake_from}");
    let plural_snake_link_type_name = format_ident!(
        "{}",
        pluralizer::pluralize(&link_type_name.to_case(Case::Snake), 2, false)
    );
    let get_link_type_function_name =
        format_ident!("get_{plural_snake_link_type_name}_for_{snake_from}");

    quote! {
          use hdk::prelude::*;
          use #integrity_zome_name::*;

          #[derive(Serialize, Deserialize, Debug)]
          pub struct #create_link_input_struct_name {
              pub #snake_from_arg: #from_field_type,
              pub #snake_link_type_name: String,
          }

          #[hdk_extern]
          pub fn #create_link_function_name(input: #create_link_input_struct_name) -> ExternResult<()> {
              create_link(
                  input.#snake_from_arg.clone(),
                  input.#snake_from_arg,
                  LinkTypes::#pascal_link_type_name,
                  input.#snake_link_type_name,
              )?;
              Ok(())
          }

          #[hdk_extern]
          pub fn #get_link_type_function_name(#snake_from_arg: #from_field_type) -> ExternResult<Vec<String>> {
              let links = get_links(
                  GetLinksInputBuilder::try_new(#snake_from_arg, LinkTypes::#pascal_link_type_name)?.build(),
              )?;
              let #snake_link_type_name = links
                  .into_iter()
                  .map(|link|
                      String::from_utf8(link.tag.into_inner())
                          .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Error converting link tag to string: {:?}", e))))
                  )
                  .collect::<ExternResult<Vec<String>>>()?;
              Ok(#snake_link_type_name)
          }
    }
}

pub fn add_link_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    bidirectional: bool,
) -> TokenStream {
    let from_field_type = format_ident!("{}", from_referenceable.field_type().to_string());
    let to_field_type = format_ident!("{}", to_referenceable.field_type().to_string());
    let target_field_name = format_ident!(
        "target_{}",
        to_referenceable.field_name(&Cardinality::Single)
    );
    let base_field_name = format_ident!(
        "base_{}",
        from_referenceable.field_name(&Cardinality::Single)
    );
    let add_link_input_struct_name = format_ident!(
        "Add{}For{}Input",
        to_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Pascal),
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Pascal)
    );
    let add_link_function_name = format_ident!(
        "add_{}_for_{}",
        to_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake),
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake)
    );

    let normal_link_type_name =
        format_ident!("{}", link_type_name(from_referenceable, to_referenceable));
    let inverse_link_type_name =
        format_ident!("{}", link_type_name(to_referenceable, from_referenceable));

    let bidirectional_create = bidirectional
        .then(|| {
            quote! {
                create_link(
                    input.#target_field_name,
                    input.#base_field_name,
                    LinkTypes::#inverse_link_type_name,
                    (),
                )?;
            }
        })
        .unwrap_or_default();

    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct #add_link_input_struct_name {
            pub #base_field_name: #from_field_type,
            pub #target_field_name: #to_field_type,
        }

        #[hdk_extern]
        pub fn #add_link_function_name(input: #add_link_input_struct_name) -> ExternResult<()> {
            create_link(
                input.#base_field_name.clone(),
                input.#target_field_name.clone(),
                LinkTypes::#normal_link_type_name,
                (),
            )?;
            #bidirectional_create
            Ok(())
        }
    }
}

pub fn get_links_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    delete: bool,
) -> TokenStream {
    match to_referenceable {
        Referenceable::Agent { .. } => {
            get_links_to_agent_handler(from_referenceable, to_referenceable, delete)
        }
        Referenceable::ExternalHash { .. } => {
            get_links_to_any_linkable_hash_handler(from_referenceable, to_referenceable, delete)
        }
        Referenceable::EntryType(e) => get_links_to_entry_handler(from_referenceable, e, delete),
    }
}

fn get_links_to_agent_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    delete: bool,
) -> TokenStream {
    let from_field_type = format_ident!("{}", from_referenceable.field_type().to_string());
    let from_arg_name = format_ident!("{}", from_referenceable.field_name(&Cardinality::Single));

    let pascal_link_type_name =
        format_ident!("{}", link_type_name(from_referenceable, to_referenceable));
    let singular_snake_from_entry_type = format_ident!(
        "{}",
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake)
    );
    let plural_snake_to_entry_type = format_ident!(
        "{}",
        to_referenceable
            .to_string(&Cardinality::Vector)
            .to_case(Case::Snake)
    );

    let get_deleted_entry_for_entry_function_name = format_ident!(
        "get_deleted_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}"
    );

    let get_deleted_links_handler = delete
        .then(|| {
            quote::quote! {
                #[hdk_extern]
                pub fn #get_deleted_entry_for_entry_function_name(
                    #from_arg_name: #from_field_type,
                ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
                    let details = get_link_details(
                        #from_arg_name,
                        LinkTypes::#pascal_link_type_name,
                        None,
                        GetOptions::default(),
                    )?;
                    Ok(details
                        .into_inner()
                        .into_iter()
                        .filter(|(_link, deletes)| !deletes.is_empty())
                        .collect())
                }
            }
        })
        .unwrap_or_default();

    let get_entry_for_entry_function_name =
        format_ident!("get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}");

    quote::quote! {
        #[hdk_extern]
        pub fn #get_entry_for_entry_function_name(#from_arg_name: #from_field_type) -> ExternResult<Vec<Link>> {
            get_links(
                GetLinksInputBuilder::try_new(#from_arg_name, LinkTypes::#pascal_link_type_name)?.build(),
            )
        }

        #get_deleted_links_handler
    }
}

fn get_links_to_entry_handler(
    from_referenceable: &Referenceable,
    to_entry_type: &EntryTypeReference,
    delete: bool,
) -> TokenStream {
    let from_field_type = format_ident!("{}", from_referenceable.field_type().to_string());
    let from_arg_name = format_ident!("{}", from_referenceable.field_name(&Cardinality::Single));

    let pascal_link_type_name = format_ident!(
        "{}",
        link_type_name(
            from_referenceable,
            &Referenceable::EntryType(to_entry_type.clone()),
        )
    );
    let singular_snake_from_entry_type = format_ident!(
        "{}",
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake)
    );
    let plural_snake_to_entry_type = format_ident!(
        "{}",
        to_entry_type
            .name_by_cardinality(&Cardinality::Vector)
            .to_case(Case::Snake)
    );

    let get_deleted_entry_for_entry_function_name = format_ident!(
        "get_deleted_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}"
    );

    let get_deleted_links_handler = delete
        .then(|| {
            quote::quote! {
                #[hdk_extern]
                pub fn #get_deleted_entry_for_entry_function_name(
                    #from_arg_name: #from_field_type,
                ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
                    let details = get_link_details(
                        #from_arg_name,
                        LinkTypes::#pascal_link_type_name,
                        None,
                        GetOptions::default(),
                    )?;
                    Ok(details
                        .into_inner()
                        .into_iter()
                        .filter(|(_link, deletes)| !deletes.is_empty())
                        .collect())
                }
            }
        })
        .unwrap_or_default();

    let get_entry_for_entry_function_name =
        format_ident!("get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}");

    quote::quote! {
        #[hdk_extern]
        pub fn #get_entry_for_entry_function_name(#from_arg_name: #from_field_type) -> ExternResult<Vec<Link>> {
            get_links(
                GetLinksInputBuilder::try_new(#from_arg_name, LinkTypes::#pascal_link_type_name)?.build(),
            )
        }

        #get_deleted_links_handler
    }
}

fn get_links_to_any_linkable_hash_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    deletable: bool,
) -> TokenStream {
    let from_field_type = format_ident!("{}", from_referenceable.field_type().to_string());
    let from_arg_name = format_ident!("{}", from_referenceable.field_name(&Cardinality::Single));

    let pascal_link_type_name =
        format_ident!("{}", link_type_name(from_referenceable, to_referenceable));
    let singular_snake_from_entry_type = format_ident!(
        "{}",
        from_referenceable
            .to_string(&Cardinality::Single)
            .to_case(Case::Snake)
    );
    let plural_snake_to_entry_type = format_ident!(
        "{}",
        to_referenceable
            .to_string(&Cardinality::Vector)
            .to_case(Case::Snake)
    );

    let get_deleted_entry_for_entry_function_name = format_ident!(
        "get_deleted_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}"
    );

    let get_deleted_links_handler = deletable
        .then(|| {
            quote::quote! {
                #[hdk_extern]
                pub fn #get_deleted_entry_for_entry_function_name(
                    #from_arg_name: #from_field_type,
                ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
                    let details = get_link_details(
                        #from_arg_name,
                        LinkTypes::#pascal_link_type_name,
                        None,
                        GetOptions::default(),
                    )?;
                    Ok(details
                        .into_inner()
                        .into_iter()
                        .filter(|(_link, deletes)| !deletes.is_empty())
                        .collect())
                }
            }
        })
        .unwrap_or_default();

    let get_entry_for_entry_function_name =
        format_ident!("get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}");

    quote::quote! {
        #[hdk_extern]
        pub fn #get_entry_for_entry_function_name(#from_arg_name: #from_field_type) -> ExternResult<Vec<Link>> {
            get_links(
                GetLinksInputBuilder::try_new(#from_arg_name, LinkTypes::#pascal_link_type_name)?.build(),
            )
        }

        #get_deleted_links_handler
    }
}

fn remove_link_handlers(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    bidirectional: bool,
) -> TokenStream {
    let from_field_type = from_referenceable.field_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);

    let inverse_link_type_name =
        format_ident!("{}", link_type_name(to_referenceable, from_referenceable));
    let singular_pascal_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let singular_pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);

    let to_arg_name = format_ident!("{}", to_referenceable.field_name(&Cardinality::Single));

    let remove_link_for_link_struct_name = format_ident!(
        "Remove{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input"
    );
    let base_field_name = format_ident!("base_{from_arg_name}");
    let from_field_type = format_ident!("{from_field_type}");
    let target_field_name = format_ident!("target_{to_arg_name}");
    let to_field_type = format_ident!("{}", to_referenceable.field_type().to_string());

    let delete_link_for_link_function_name =
        format_ident!("delete_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}");
    let pascal_link_type_name =
        format_ident!("{}", link_type_name(from_referenceable, to_referenceable));

    let from_link_hash_type_code = hash_type_code_from_referenceable(to_referenceable);

    let bidirectional_remove = bidirectional
        .then(|| {
            let from_inverse_hash_type = hash_type_code_from_referenceable(from_referenceable);

            quote! {
                let links = get_links(
                    GetLinksInputBuilder::try_new(
                        input.#target_field_name.clone(),
                        LinkTypes::#inverse_link_type_name)?.build(),
                )?;
                for link in links {
                    if #from_inverse_hash_type == input.#base_field_name.clone().into_hash().into() {
                        delete_link(link.create_link_hash)?;
                    }
                }
            }
        })
        .unwrap_or_default();

    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct #remove_link_for_link_struct_name {
            pub #base_field_name: #from_field_type,
            pub #target_field_name: #to_field_type,
        }

        #[hdk_extern]
        pub fn #delete_link_for_link_function_name(input: #remove_link_for_link_struct_name) -> ExternResult<()> {
            let links = get_links(
                GetLinksInputBuilder::try_new(input.#base_field_name.clone(), LinkTypes::#pascal_link_type_name)?.build(),
            )?;
            for link in links {
                if #from_link_hash_type_code == input.#target_field_name.clone().into_hash().into() {
                    delete_link(link.create_link_hash)?;
                }
            }
            #bidirectional_remove
            Ok(())
        }
    }
}

fn hash_type_code_from_referenceable(referenceable: &Referenceable) -> TokenStream {
    match referenceable {
        Referenceable::Agent { .. } => quote! {
            AgentPubKey::from(
                link.target.clone()
                    .into_entry_hash()
                    .ok_or(wasm_error!(
                        WasmErrorInner::Guest("No entry_hash associated with link".to_string())
                    ))?
            )
        },
        Referenceable::ExternalHash { .. } => quote! {
            link.target.clone().into_hash()
        },
        Referenceable::EntryType(_) => {
            let field_type = referenceable.field_type().to_string();
            let into_hash_method_name = format_ident!("into_{}", field_type.to_case(Case::Snake));
            let error_message = format!(
                "No {} associated with link",
                field_type.to_case(Case::Lower)
            );

            quote! {
                link.target
                    .clone()
                    .#into_hash_method_name()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(#error_message.to_string())))?
            }
        }
    }
}
