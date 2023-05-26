use anyhow::{Context, Result};
use boringtun::noise::{Tunn, TunnResult};
use std::io::{Read, Write};
use std::sync::Arc;
pub use x25519_dalek::{PublicKey, StaticSecret};

use tokio::net::UdpSocket;

const MTU: usize = 4096;

pub struct TunnelConfig {
    pub keepalive_seconds: u16,
    pub peer_public_key: PublicKey,
    pub private_key: StaticSecret,
}

pub struct Tunnel {
    udp_socket: Arc<UdpSocket>,
    tunn: Tunn,
    virtual_iface: tun::AsyncDevice,
}

impl Tunnel {
    /// Creates a new wireguard tunnel with the given peer information
    pub async fn new(cfg: TunnelConfig) -> Result<Self> {
        // Create a tunnel
        let tunn = Self::create_tunnel(cfg).context("error making wireguard tunnel")?;

        // Create a tun device
        let mut config = tun::Configuration::default();
        config
            .address((10, 0, 0, 1))
            .destination((10, 0, 0, 1))
            .netmask((255, 255, 255, 0))
            .mtu(4096)
            .up(); // TODO: Also figure a name

        // The tun/tap device
        let dev =
            tun::create_as_async(&config).context("error creating virtual network interface")?;

        // A _real_ socket for writing to our peer
        let udp_socket = UdpSocket::bind("0.0.0.0").await?;

        Ok(Self {
            udp_socket: Arc::new(udp_socket),
            tunn,
            virtual_iface: dev,
        })
    }

    // Makes a wireguard tunnel
    fn create_tunnel(cfg: TunnelConfig) -> Result<Tunn> {
        Tunn::new(
            cfg.private_key,
            cfg.peer_public_key,
            None,
            Some(cfg.keepalive_seconds),
            0,
            None,
        )
        .map_err(|s| anyhow::anyhow!("{}", s))
        .with_context(|| "Failed to initialize boringtun Tunn")
    }

    // Starts both loops for reading and forwarding
    pub async fn start(&mut self) -> Result<()> {
        // Two loops:
        // 1: Receives on the _real_ udp socket and then decapsulate and write the tun device
        // 2: Listens on the tun device, encapsulates, and writes to the udp socket

        let listener_sock = self.udp_socket.clone();
        let listener_handle = tokio::spawn(async move {
            let mut buf = [0; MTU];
            let mut send_buf = [0; MTU];

            loop {
                let (len, addr) = listener_sock
                    .recv_from(&mut buf)
                    .await
                    .expect("error receiving from socket");

                println!("{:?} bytes received from {:?}", len, addr);

                match self.tunn.decapsulate(None, &buf[..len], &mut send_buf) {};
            }
        });

        let writer_sock = self.udp_socket.clone();
        let writer_handle = tokio::spawn(async move {});

        Ok(())
    }
}
