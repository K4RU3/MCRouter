mod config;
mod packet;
mod proxy;
mod handler;

use crate::config::load_and_build_domain_map;
use crate::handler::handle_connection;
use tokio::net::TcpListener;
use clap::Parser;

/// コマンドライン引数のパーサ
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 使用するポート番号（デフォルト: 25565）
    #[arg(long, default_value_t = 25565)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain_map = match load_and_build_domain_map("config.toml") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("設定ファイル読み込みエラー: {}", e);
            return Err(e);
        }
    };

    let args = Args::parse();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    println!("TCPプロキシサーバーをポート{}で起動...", args.port);

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
