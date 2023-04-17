use anyhow::Result;
use clap::{arg, Command};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

fn cli() -> Command {
    Command::new("diver")
        .about("The main diver client")
        .subcommand_required(true)
        .subcommand(Command::new("send").about("Sends a file"))
        .subcommand(Command::new("recv").about("Receives a file"))
        .arg(arg!(-r --remote <port> "the remote port for testing purposes").required(true))
        .arg(arg!(-p --port <port> "the host port for testing purposes").required(true))
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    // DEBUG: make a socket to listen on, and determine the remote address
    let sock = UdpSocket::bind((
        "127.0.0.1",
        matches
            .get_one::<String>("port")
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    ))
    .await?;
    let remote: SocketAddr = format!(
        "127.0.0.1:{}",
        matches
            .get_one::<String>("remote")
            .unwrap()
            .parse::<u16>()
            .unwrap()
    )
    .parse()?;

    match matches.subcommand() {
        Some(("send", submatches)) => {
            // TODO: Open the file, start sending over wireguard
            // Hole punch it up
            let sock = diver::perform_holepunch(sock, remote).await?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

// TODO: Make a function that generates a private/public key pair
