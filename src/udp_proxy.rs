use tokio::net::UdpSocket;

pub async fn udp_proxy(
  bind_address: String,
  target_address: String,
) -> anyhow::Result<()> {
  let local = UdpSocket::bind(&target_address).await?;
  let remote = UdpSocket::bind(&bind_address).await?;

  tokio::spawn(async move {
    let mut buf = [0u8; 65535];

    loop {
      match local.recv_from(&mut buf).await {
        Ok((size, _src)) => {
          let data = buf[..size].to_vec();
          if remote.send(&data).await.is_err() {
            eprintln!("[UDP] Unable to forward data to {}", bind_address);
            continue;
          };
        }
        Err(_err) => {
          eprintln!("[UDP] Unable to receive data from {}", target_address);
          break;
        }
      }
    }
  });

  Ok(())
}
