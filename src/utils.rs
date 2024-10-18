use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::time::Duration;

/// Wrapper around a UDP socket.
#[derive(Debug)]
pub struct UdpSocketWrapper {
    socket: UdpSocket,
}

impl UdpSocketWrapper {
    /// Create a socket wrapper for connections to the specified address.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the binding of the inner socket fails.
    pub fn connect(address: SocketAddr) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(if address.is_ipv4() {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
        } else {
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
        })?;
        socket.connect(address)?;
        Ok(Self { socket })
    }

    /// Send data via the UDP socket.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the sending fails.
    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.socket.send(data)
    }

    /// Receive data from the UDP socket.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the receiving fails.
    pub fn recv(&self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buffer)
    }

    /// Sets the read timeout.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if setting the timeout fails.
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> std::io::Result<()> {
        self.socket.set_read_timeout(timeout)
    }

    /// Sets the write timeout.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if setting the timeout fails.
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> std::io::Result<()> {
        self.socket.set_write_timeout(timeout)
    }
}
