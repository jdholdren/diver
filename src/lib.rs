use anyhow::{anyhow, Result};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str;
use tokio::net::UdpSocket;
use tokio::time;

const HOLE_PUNCH_MAX_TRIES: u16 = 10;

pub async fn holepunch(remote_addr: SocketAddr) -> Result<UdpSocket> {
    // The '0' port randomly assigns an unused port
    let sock = UdpSocket::bind(("127.0.0.1", 0)).await?;
    perform_holepunch(sock, remote_addr).await
}

// Performs the holepunch and returns a socket that has successfully made it out
async fn perform_holepunch(sock: UdpSocket, remote_addr: SocketAddr) -> Result<UdpSocket> {
    println!("We're listening on: {}", sock.local_addr()?.port());

    let mut rng = StdRng::from_entropy();
    // Generate some number to be our seed to share with the remote
    let seed = rng.gen_range(1..4444);
    // We initially don't know the remote's seed
    let mut known_seed: Option<u16> = None;

    let mut buf = [0; 1024];

    for _i in 0..HOLE_PUNCH_MAX_TRIES {
        // Start with a send
        let msg = HolePunchMessage { seed, known_seed };
        let marshalled = serde_json::to_string(&msg)?;
        sock.send_to(marshalled.as_bytes(), remote_addr).await?;

        // Then wait to receive with some jitter
        if let Ok(Ok(msg_size)) = time::timeout(
            time::Duration::from_millis(1000 + rng.gen_range(0..100)),
            sock.recv(&mut buf),
        )
        .await
        {
            // TODO: Clear the buffer
            println!("Buffer: {:?}", buf);
            // We get a response parse it out
            let msg: HolePunchMessage = match serde_json::from_slice(&buf[0..msg_size]) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("error deserialzing: {}", err);
                    continue;
                }
            };
            println!("Got message: {:?}", msg);
            // If we don't have a known seed, set it and continue
            if known_seed.is_none() {
                known_seed = Some(msg.seed);
                continue;
            }
            // If we do have a seed, make sure it matches
            if known_seed.unwrap() != msg.seed {
                return Err(anyhow!("seeds didn't match"));
            }
            // Check if the remote knows our seed
            if msg.known_seed.unwrap_or(0) == seed {
                // The remote knows us, we're done!
                return Ok(sock);
            }
        };

        println!("Trying again...");
    }

    Err(anyhow!("too many tries holepunching"))
}

// Represents what each client sends during the holepunch
#[derive(Serialize, Deserialize, Debug)]
struct HolePunchMessage {
    seed: u16,               // The client's seed that it's sending
    known_seed: Option<u16>, // If the client knows the other's secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn holepunch_connects() {
        // The addresses for each client
        let addr_1: SocketAddr = "127.0.0.1:4444".parse().unwrap();
        let addr_2: SocketAddr = "127.0.0.1:4445".parse().unwrap();
        let sock_1 = UdpSocket::bind(addr_1).await.unwrap();
        let sock_2 = UdpSocket::bind(addr_2).await.unwrap();

        let handle_1 = tokio::spawn(async move {
            perform_holepunch(sock_1, addr_2).await.unwrap();
        });
        let handle_2 = tokio::spawn(async move {
            perform_holepunch(sock_2, addr_1).await.unwrap();
        });

        // Wait for both
        tokio::join!(handle_1, handle_2).0.unwrap();
    }
}
