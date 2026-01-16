use crate::error::{ScaffoldError, ScaffoldResult};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ExampleType {
    HelloWorld,
    Forum,
}

impl ExampleType {
    pub fn choose() -> ScaffoldResult<Self> {
        let examples = [ExampleType::Forum, ExampleType::HelloWorld];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose example:")
            .items(&examples)
            .default(0)
            .interact()?;

        Ok(examples[selection].clone())
    }
}

impl std::fmt::Display for ExampleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ExampleType::HelloWorld => "hello-world",
            ExampleType::Forum => "forum",
        };
        write!(f, "{str}")
    }
}

impl FromStr for ExampleType {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<Self> {
        match s {
            "hello-world" => Ok(ExampleType::HelloWorld),
            "forum" => Ok(ExampleType::Forum),
            _ => Err(ScaffoldError::InvalidExampleType(
                s.to_string(),
                "hello-world, forum".to_string(),
            )),
        }
    }
}
