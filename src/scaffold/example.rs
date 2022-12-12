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

impl ToString for Example {
    fn to_string(&self) -> String {
        match self {
            Example::HelloWorld => String::from("hello-world"),
            Example::Forum => String::from("forum"),
        }
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
