use std::path::PathBuf;

use clap::Parser;
use normalize_path::NormalizePath;

#[derive(Debug, Parser)]
pub struct Command {
  #[arg(short = 'c', long = "config", default_value = "./spf.toml")]
  pub config: PathBuf,
}

impl Command {
  pub fn from_argv() -> anyhow::Result<Command> {
    let mut cmd = Command::parse();

    if cmd.config.is_relative() {
      cmd.config = std::env::current_dir()?.join(cmd.config)
    }

    cmd.config = cmd.config.normalize();

    Ok(cmd)
  }
}
