use tokio::io;
use tokio::net::TcpStream;

pub async fn start_proxy(mut inbound: TcpStream, forward_addr: String) -> io::Result<()> {
    let mut outbound = TcpStream::connect(forward_addr).await?;

    io::copy_bidirectional(&mut inbound, &mut outbound).await?;
    Ok(())
}
