use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn tcp_proxy(
  bind_address: String,
  target_address: String,
) -> anyhow::Result<()> {
  let listener = TcpListener::bind(bind_address).await?;

  loop {
    let (client, _) = listener.accept().await?;
    let target_address = target_address.clone();

    tokio::spawn({
      let target_address = target_address.clone();

      async move {
        let Ok(server) = TcpStream::connect(&target_address).await else {
          eprintln!("Error connecting to: {}", target_address);
          return;
        };

        let (mut eread, mut ewrite) = client.into_split();
        let (mut oread, mut owrite) = server.into_split();

        tokio::spawn({
          let target_address = target_address.clone();
          async move {
            if io::copy(&mut eread, &mut owrite).await.is_err() {
              eprintln!("Error copying to: {}", target_address);
            }
          }
        });

        tokio::spawn({
          let target_address = target_address.clone();
          async move {
            if io::copy(&mut oread, &mut ewrite).await.is_err() {
              eprintln!("Error copying from: {}", target_address);
            }
          }
        });
      }
    });
  }
}
