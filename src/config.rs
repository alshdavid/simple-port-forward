use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub binding: Vec<ConfigBinding>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigBinding {
  pub target: String,
  pub bind: String,
  #[serde(default)]
  pub protocol: ConfigProto,
}

#[derive(Debug, Deserialize, Default)]
pub enum ConfigProto {
  #[default]
  #[allow(clippy::upper_case_acronyms)]
  TCP,
  #[allow(clippy::upper_case_acronyms)]
  UDP,
  #[allow(clippy::upper_case_acronyms)]
  HTTP,
}

impl Display for ConfigProto {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ConfigProto::TCP => write!(f, "TCP"),
      ConfigProto::UDP => write!(f, "UDP"),
      ConfigProto::HTTP => write!(f, "HTTP"),
    }
  }
}
