use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn tcp_proxy(
  to_address: String,
  from_address: String,
) -> anyhow::Result<()> {
  let listener = TcpListener::bind(to_address).await?;

  loop {
    let (client, _) = listener.accept().await?;
    let server = from_address.clone();

    tokio::spawn(async move {
      tcp_proxy_socket(server, client).await.ok();
    });
  }
}

async fn tcp_proxy_socket(
  server: String,
  client: TcpStream,
) -> anyhow::Result<()> {
  let server = TcpStream::connect(server).await?;

  let (mut eread, mut ewrite) = client.into_split();
  let (mut oread, mut owrite) = server.into_split();

  tokio::spawn(async move {
    io::copy(&mut eread, &mut owrite).await.unwrap();
  });

  tokio::spawn(async move {
    io::copy(&mut oread, &mut ewrite).await.unwrap();
  });

  Ok(())
}
