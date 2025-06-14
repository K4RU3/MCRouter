mod config;
mod packet;
mod proxy;
mod handler;

use crate::config::load_and_build_domain_map;
use crate::handler::handle_connection;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain_map = match load_and_build_domain_map("config.toml") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("設定ファイル読み込みエラー: {}", e);
            return Err(e);
        }
    };

    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    println!("TCPプロキシサーバ起動...");

    loop {
        let (socket, _) = listener.accept().await?;
        let domain_map = domain_map.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, domain_map).await {
                eprintln!("接続エラー: {}", e);
            }
        });
    }
}
