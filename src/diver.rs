use anyhow::Result;
use clap::{arg, Command};
use std::net::SocketAddr;

use diver::holepunch;

fn cli() -> Command {
    Command::new("diver")
        .about("The main diver client")
        .subcommand_required(true)
        .subcommand(Command::new("send").about("Sends a file"))
        .subcommand(Command::new("recv").about("Receives a file"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("send", submatches)) => {
            let remote: SocketAddr = "127.0.0.1:4444".parse().expect("error parsing remote");
            // TODO: Open the file, start sending over wireguard
            // Hole punch it up
            holepunch(remote).await?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

// TODO: Make a function that generates a private/public key pair
