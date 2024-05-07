use crate::error::{ScaffoldError, ScaffoldResult};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Example {
    HelloWorld,
    Forum,
}

impl std::fmt::Display for Example {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Example::HelloWorld => "hello-world",
            Example::Forum => "forum",
        };
        write!(f, "{str}")
    }
}

impl FromStr for Example {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<Self> {
        match s {
            "hello-world" => Ok(Example::HelloWorld),
            "forum" => Ok(Example::Forum),
            _ => Err(ScaffoldError::InvalidExampleType(
                s.to_string(),
                "hello-world, forum".to_string(),
            )),
        }
    }
}

pub fn choose_example() -> ScaffoldResult<Example> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose example:")
        .item("hello-world")
        .item("forum")
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(Example::HelloWorld),
        _ => Ok(Example::Forum),
    }
}
