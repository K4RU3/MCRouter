use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use crate::packet::{parse_first_packet, create_login_disconnect_packet, generate_unknown_domain_message};
use crate::proxy::start_proxy;

pub async fn handle_connection(mut stream: TcpStream, domain_map: HashMap<String, String>) -> tokio::io::Result<()> {
    let (first_packet, full_packet_buf) = parse_first_packet(&mut stream).await?;
    println!("初回パケット: {:?}", first_packet);

    if first_packet.packet_id != 0 {
        return Ok(());
    }

    if let Some(forward_addr) = domain_map.get(&first_packet.domain) {
        println!("転送先: {}", forward_addr);
        start_proxy(stream, forward_addr.clone(), full_packet_buf).await?;
    } else {
        println!("不明ドメイン: {}", first_packet.domain);

        let message = generate_unknown_domain_message(&first_packet.domain, &domain_map);
        let disconnect_packet = create_login_disconnect_packet(&message);
        stream.write_all(&disconnect_packet).await?;
        stream.shutdown().await?;
    }
    Ok(())
}
