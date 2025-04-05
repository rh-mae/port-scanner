# Rust Port Scanner

A simple asynchronous TCP port scanner written in Rust using `tokio` and `clap`.  
This tool allows scanning a range of ports on a given IP address and reports which ports are open.

## Features

- Asynchronous and fast port scanning using Tokio
- CLI interface with argument parsing via Clap
- Timeout-based detection for responsiveness
- Works cross-platform

## Usage

To run the scanner:

```bash
cargo run -- --ip <IP_ADDRESS> --start-port <START> --end-port <END>

Example:
cargo run -- --ip 127.0.0.1 --start-port 20 --end-port 100
