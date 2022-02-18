import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const handlersRs = ({moduleNameSnakeCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNameTitleCase: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `use crate::{utils, Agent${moduleNameTitleCase}, ${moduleNameTitleCase}};
use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;
use std::convert::TryInto;

pub fn create${moduleNameSnakeCase}(${snakeCase(moduleName)}: ${moduleNameTitleCase}) -> ExternResult<Agent${moduleNameTitleCase}> {
    let agent_info = agent_info()?;

    create_entry(&${snakeCase(moduleName)}.clone())?;

    let ${snakeCase(moduleName)}_hash = hash_entry(&${snakeCase(moduleName)}.clone())?;

    let path = prefix_path(${snakeCase(moduleName)}.nickname.clone());

    path.ensure()?;

    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    create_link(
        path.path_entry_hash()?,
        ${snakeCase(moduleName)}_hash.clone(),
        link_tag(${snakeCase(moduleName)}.nickname.as_str().clone())?,
    )?;
    create_link(
        agent_address.into(),
        ${snakeCase(moduleName)}_hash.clone(),
        link_tag("${snakeCase(moduleName)}")?,
    )?;

    let agent${moduleNameSnakeCase} = Agent${moduleNameTitleCase} {
        agent_pub_key: AgentPubKeyB64::from(agent_info.agent_initial_pubkey),
        ${snakeCase(moduleName)},
    };

    Ok(agent${moduleNameSnakeCase})
}

pub fn update${moduleNameSnakeCase}(${snakeCase(moduleName)}: ${moduleNameTitleCase}) -> ExternResult<Agent${moduleNameTitleCase}> {
    let agent_info = agent_info()?;

    create_entry(&${snakeCase(moduleName)}.clone())?;

    let ${snakeCase(moduleName)}_hash = hash_entry(&${snakeCase(moduleName)}.clone())?;

    let path = prefix_path(${snakeCase(moduleName)}.nickname.clone());

    path.ensure()?;

    let agent_address = agent_info.agent_initial_pubkey.clone();

    let link_details = get_link_details(path.path_entry_hash()?, None)?.into_inner();

    if link_details.len() > 0 {
        // check whether the agent has committed a ${snakeCase(moduleName)} before
        // needs to be checked because duplicate ${moduleNameTitleCase} is possible
        let ${snakeCase(moduleName)}_exist = link_details
            .clone()
            .into_iter()
            .find(|detail| detail.0.header().author().to_owned() == agent_address)
            .is_some();
        if ${snakeCase(moduleName)}_exist {
            link_details
                .clone()
                .into_iter()
                .filter_map(|detail| {
                    let is_my${moduleNameSnakeCase} = detail.0.header().author().to_owned() == agent_address;
                    let is_not_deleted = detail.1.is_empty();
                    if is_my${moduleNameSnakeCase} && is_not_deleted {
                        return Some(detail.0.as_hash().to_owned());
                    } else {
                        return None;
                    }
                })
                .for_each(|header| {
                    // ignore error
                    match delete_link(header) {
                        Ok(_) => (),
                        // TODO: probably should return error once one of the delete fails
                        Err(_) => (),
                    }
                });
        }
    }

    let links = get_links(agent_address.clone().into(), Some(link_tag("${snakeCase(moduleName)}")?))?;
    if links.len() > 0 {
        let link = links[0].clone();
        delete_link(link.create_link_hash)?;
    }

    create_link(
        path.path_entry_hash()?,
        ${snakeCase(moduleName)}_hash.clone(),
        link_tag(${snakeCase(moduleName)}.nickname.as_str().clone())?,
    )?;
    create_link(
        agent_address.into(),
        ${snakeCase(moduleName)}_hash.clone(),
        link_tag("${snakeCase(moduleName)}")?,
    )?;

    let agent${moduleNameSnakeCase} = Agent${moduleNameTitleCase} {
        agent_pub_key: AgentPubKeyB64::from(agent_info.agent_initial_pubkey),
        ${snakeCase(moduleName)},
    };

    Ok(agent${moduleNameSnakeCase})
}

pub fn search${moduleNameSnakeCase}s(nickname_prefix: String) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    if nickname_prefix.len() < 3 {
        return Err(utils::err(
            "Cannot search with a prefix less than 3 characters",
        ));
    }

    let prefix_path = prefix_path(nickname_prefix);

    get_agent${moduleNameSnakeCase}s_for_path(prefix_path.path_entry_hash()?)
}

pub fn get_all${moduleNameSnakeCase}s() -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let path = Path::from("all${moduleNameSnakeCase}s");

    let children = path.children()?;

    let agent${moduleNameSnakeCase}s: Vec<Agent${moduleNameTitleCase}> = children
        .into_iter()
        .map(|link| get_agent${moduleNameSnakeCase}s_for_path(link.target))
        .collect::<ExternResult<Vec<Vec<Agent${moduleNameTitleCase}>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(agent${moduleNameSnakeCase}s)
}

pub fn get_agent${moduleNameSnakeCase}(
    wrapped_agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<Agent${moduleNameTitleCase}>> {
    let agent_pub_key = AgentPubKey::from(wrapped_agent_pub_key.clone());

    let agent_address: AnyDhtHash = agent_pub_key.into();

    let links = get_links(agent_address.into(), Some(link_tag("${snakeCase(moduleName)}")?))?;

    if links.len() == 0 {
        return Ok(None);
    }

    let link = links[0].clone();

    let ${snakeCase(moduleName)}: ${moduleNameTitleCase} = utils::try_get_and_convert(link.target)?;

    let agent${moduleNameSnakeCase} = Agent${moduleNameTitleCase} {
        agent_pub_key: wrapped_agent_pub_key,
        ${snakeCase(moduleName)},
    };

    Ok(Some(agent${moduleNameSnakeCase}))
}

pub fn get_agents${moduleNameSnakeCase}(
    agent_pub_keys_b64: Vec<AgentPubKeyB64>,
) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let link_tag = Some(link_tag("${snakeCase(moduleName)}")?);

    let get_links_input: Vec<GetLinksInput> = agent_pub_keys_b64
        .into_iter()
        .map(|agent_pub_key_b64| {
            let agent_pub_key = AgentPubKey::from(agent_pub_key_b64.clone());
            let agent_address: AnyDhtHash = agent_pub_key.into();
            GetLinksInput::new(agent_address.into(), link_tag.clone())
        })
        .collect();

    let get_links_output = HDK
        .with(|h| h.borrow().get_links(get_links_input))?
        .into_iter()
        .flatten()
        .collect::<Vec<Link>>();

    let get_input = get_links_output
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();
    let get_output = HDK.with(|h| h.borrow().get(get_input))?;

    get_output
        .into_iter()
        .filter_map(|maybe_option| maybe_option)
        .map(get_agent${moduleNameSnakeCase}_from_element)
        .collect()
}

/** Private helpers */

fn prefix_path(nickname: String) -> Path {
    // conver to lowercase for path for ease of search
    let lower_nickname = nickname.to_lowercase();
    let (prefix, _) = lower_nickname.as_str().split_at(3);

    Path::from(format!("all${moduleNameSnakeCase}s.{}", prefix))
}

fn get_agent${moduleNameSnakeCase}s_for_path(path_hash: EntryHash) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let links = get_links(path_hash, None)?;

    let get_input = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let get_output = HDK.with(|h| h.borrow().get(get_input))?;

    get_output
        .into_iter()
        .filter_map(|maybe_option| maybe_option)
        .map(get_agent${moduleNameSnakeCase}_from_element)
        .collect()
}

fn get_agent${moduleNameSnakeCase}_from_element(element: Element) -> ExternResult<Agent${moduleNameTitleCase}> {
    let author = element.header().author().clone();

    let ${snakeCase(moduleName)}: ${moduleNameTitleCase} = utils::try_from_element(element)?;

    let agent${moduleNameSnakeCase} = Agent${moduleNameTitleCase} {
        agent_pub_key: AgentPubKeyB64::from(author),
        ${snakeCase(moduleName)},
    };

    Ok(agent${moduleNameSnakeCase})
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
`
});
    