// use clap::Error;
// use clap::{arg, Command};

// fn cli() -> Command {
//     Command::new("diver")
//         .about("The main diver client")
//         .subcommand_required(true)
//         .subcommand(Command::new("send").about("Sends a file"))
//         .subcommand(Command::new("recv").about("Receives a file"))
//         .subcommand(Command::new("mid").about("The mid thing"))
//     // .arg(arg!(-r --remote <port> "the remote port for testing purposes").required(true))
//     // .arg(arg!(-p --port <port> "the host port for testing purposes").required(true))
// }

#[tokio::main]
async fn main() {
    let role: String = std::env::var("ROLE").unwrap_or(String::new());

    if role == "send" {}

    if role == "recv" {}

    unreachable!();
}
