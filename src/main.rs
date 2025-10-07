mod cli;
mod config;
mod tcp_proxy;
mod udp_proxy;

use config::Config;

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

  for binding in config.tcp {
    println!("  [TCP] {} -> {}", binding.bind, binding.target);
    tasks.push(tokio::spawn(tcp_proxy::tcp_proxy(
      binding.target,
      binding.bind,
    )));
  }

  for binding in config.udp {
    println!("  [UDP] {} -> {}", binding.bind, binding.target);
    tasks.push(tokio::spawn(udp_proxy::udp_proxy(
      binding.target,
      binding.bind,
    )))
  }

  for binding in config.http {
    println!("  [HTTP] {} -> {} TODO", binding.bind, binding.target);
  }

  for binding in config.https {
    println!("  [HTTPS] {} -> {} TODO", binding.bind, binding.target);
  }

  for task in tasks {
    task.await??;
  }

  Ok(())
}
