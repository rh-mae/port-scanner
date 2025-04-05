use clap::Parser;
use tokio::net::TcpStream;
use std::net::IpAddr;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

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

