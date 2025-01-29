mod cli;
mod config;
mod tcp_proxy;

use config::Config;
use config::ConfigProto;

fn main() -> anyhow::Result<()> {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .worker_threads(num_cpus::get_physical())
    .build()
    .unwrap()
    .block_on(main_async())
}

async fn main_async() -> anyhow::Result<()> {
  let args = cli::Command::from_argv()?;
  let config_str = std::fs::read_to_string(&args.config)?;
  let config = toml::from_str::<Config>(&config_str)?;
  let mut tasks: Vec<tokio::task::JoinHandle<anyhow::Result<()>>> = vec![];

  println!("Bindings:");

  for binding in config.binding {
    println!(
      "  [{}] {} -> {}",
      binding.protocol, binding.target, binding.bind
    );
    tasks.push(tokio::spawn(async move {
      match binding.protocol {
        ConfigProto::TCP => tcp_proxy::tcp_proxy(binding.target, binding.bind).await,
        ConfigProto::UDP => todo!(),
        ConfigProto::HTTP => todo!(),
      }
    }));
  }

  for task in tasks {
    task.await??;
  }

  Ok(())
}
