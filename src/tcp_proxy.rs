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

    tokio::spawn(async move {
      let server = TcpStream::connect(target_address).await.unwrap();

      let (mut eread, mut ewrite) = client.into_split();
      let (mut oread, mut owrite) = server.into_split();

      tokio::spawn(async move {
        io::copy(&mut eread, &mut owrite).await.unwrap();
      });

      tokio::spawn(async move {
        io::copy(&mut oread, &mut ewrite).await.unwrap();
      });
    });
  }
}
