use std::str::FromStr;
use serde::Serialize;
use crate::error::{ScaffoldError, ScaffoldResult};


#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ExampleType {
  HelloWorld,
  Forum,
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