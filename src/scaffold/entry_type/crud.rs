use std::str::FromStr;

use anyhow::anyhow;
use serde::Serialize;

use crate::error::ScaffoldError;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Crud {
    // We don't include create and read because they must always exist
    pub update: bool,
    pub delete: bool,
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

        let mut crud = Crud {
            update: false,
            delete: false,
        };

        for c in crud_str.chars() {
            match c {
                'c' | 'r' => {}
                'u' => {
                    crud.update = true;
                }
                'd' => {
                    crud.delete = true;
                }
                _ => {
                    return Err(anyhow!(
                        "Only 'c', 'r', 'u' and 'd' are allowed in the crud argument",
                    )
                    .into());
                }
            }
        }

        Ok(crud)
    }
}
