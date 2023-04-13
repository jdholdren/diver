use std::net::TcpListener;

use anyhow::Result;

// TODO: Perform a holepunch
// async fn holepunch() -> Result<()> {}

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
}
