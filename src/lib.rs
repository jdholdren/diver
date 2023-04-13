use rand::Rng;
use std::net::{SocketAddr, TcpListener};
use tokio::net::UdpSocket;
use tokio::time;

const HOLE_PUNCH_MAX_TRIES: u16 = 10;

// Representing a connection that we haven't punched yet
pub struct PotentialClient {
    pub port: u16,
    sock: UdpSocket,
}

#[derive(Debug)]
pub enum PotentialClientError {
    NoFreePort,
    BindErr(std::io::Error),
}

impl PotentialClient {
    pub async fn new() -> Result<Self, PotentialClientError> {
        // Get a free port
        let Some(port) = freeport() else {
            return Err(PotentialClientError::NoFreePort);
        };

        // Get us a UDP socket
        let sock = match UdpSocket::bind(("127.0.0.1", port)).await {
            Ok(sock) => sock,
            Err(err) => return Err(PotentialClientError::BindErr(err)),
        };

        Ok(PotentialClient { port, sock })
    }

    // Performs the holepunch and tries to upgrade the potential client to a real one
    pub async fn holepunch(
        &mut self,
        remote_addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();

        let mut buf = [0; 1024];
        for _i in 0..HOLE_PUNCH_MAX_TRIES {
            // Start with a send
            self.sock.send_to(&buf, remote_addr).await?;

            // Then wait to receive with some jitter
            if time::timeout(
                time::Duration::from_millis(1000 + rng.gen_range(0..500)),
                self.sock.recv(&mut buf),
            )
            .await
            .is_ok()
            {
                println!("Got a response!");
                return Ok(());
            };
            println!("Trying again...");
        }

        Ok(())
    }
}

// Finds a free port for us to use
fn freeport() -> Option<u16> {
    // Loop through
    (49152..65535).find(|&possible| TcpListener::bind(("127.0.0.1", possible)).is_ok())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn freeport_gets_next_port() {
        // Find a free port, then take it, and we should have the next one?
        let port = freeport().expect("error finding the first open port");
        let _listener = TcpListener::bind(("127.0.0.1", port)).expect("error binding to free port");

        assert_eq!(port + 1, freeport().unwrap());
    }

    #[test]
    fn holepunch() {}
}
