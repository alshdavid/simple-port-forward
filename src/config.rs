use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub tcp: Vec<ConfigBinding>,
  #[serde(default)]
  pub udp: Vec<ConfigBinding>,
  #[serde(default)]
  pub http: Vec<ConfigBinding>,
  #[serde(default)]
  pub https: Vec<ConfigBinding>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigBinding {
  pub target: String,
  pub bind: String,
}
