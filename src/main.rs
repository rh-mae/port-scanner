use clap::Parser;
use tokio::net::TcpStream;
use std::net::IpAddr;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "🛡 Rust Port Scanner")]
struct Args {
    /// IP-адрес, который нужно просканировать
    #[arg(short, long)]
    ip: IpAddr,

    /// Начальный порт
    #[arg(short, long, default_value_t = 1)]
    start_port: u16,

    /// Конечный порт
    #[arg(short, long, default_value_t = 1024)]
    end_port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!(
        "🔍 Сканирую {} с порта {} по {}",
        args.ip, args.start_port, args.end_port
    );

    let mut handles = vec![];

    for port in args.start_port..=args.end_port {
        let ip = args.ip;
        handles.push(tokio::spawn(async move {
            let addr = format!("{}:{}", ip, port);
            let timeout = Duration::from_millis(500);

            if let Ok(Ok(_)) = tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
                println!("✅ Порт {} открыт", port);
            }
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("✅ Готово!");
}
