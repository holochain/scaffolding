use std::str::FromStr;

use anyhow::anyhow;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde::Serialize;

use crate::error::{ScaffoldError, ScaffoldResult};

#[derive(Debug, Default, Serialize, Clone, Copy)]
pub struct Crud {
    // We don't include create and read because they must always exist
    pub update: bool,
    pub delete: bool,
}

impl Crud {
    pub fn choose() -> ScaffoldResult<Self> {
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Which CRUD functions should be scaffolded (SPACE to select/unselect, ENTER to continue)?")
            .item_checked("Update", true)
            .item_checked("Delete", true)
            .interact()?;

        let crud = Crud {
            update: selections.contains(&0),
            delete: selections.contains(&1),
        };

        Ok(crud)
    }
}

impl FromStr for Crud {
    type Err = ScaffoldError;

    fn from_str(crud_str: &str) -> Result<Self, Self::Err> {
        if !crud_str.contains('c') {
            return Err(anyhow!("create ('c') must be present").into());
        }
        if !crud_str.contains('r') {
            return Err(anyhow!("read ('r') must be present").into());
        }

        let mut crud = Crud::default();

        for c in crud_str.chars() {
            match c {
                'c' | 'r' => {}
                'u' => crud.update = true,
                'd' => crud.delete = true,
                _ => {
                    return Err(anyhow!(
                        "Only 'c', 'r', 'u' and 'd' are allowed in the crud argument",
                    )
                    .into())
                }
            }
        }

        Ok(crud)
    }
}
