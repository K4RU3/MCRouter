use tokio::io;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub async fn start_proxy(mut inbound: TcpStream, forward_addr: String, initial_packet: Vec<u8>) -> io::Result<()> {
    // プロキシ先に接続
    let mut outbound = TcpStream::connect(forward_addr).await?;

    // initial_packet を送信
    outbound.write_all(&initial_packet).await?;

    // 双方向のコピー開始（以降は通常のプロキシ通信）
    io::copy_bidirectional(&mut inbound, &mut outbound).await?;

    Ok(())
}
