use std::net::SocketAddr;

use diver::PotentialClient;

#[tokio::main]
async fn main() {
    let mut pc = PotentialClient::new()
        .await
        .expect("error making potential client");

    println!("Listening on {}", pc.port);

    let remote: SocketAddr = "127.0.0.1:4444".parse().expect("error parsing remote");

    pc.holepunch(remote).await.expect("error punching a hole");
}
