use std::io;
use tokio::io::AsyncWriteExt;
use udp_stream::UdpStream;

pub trait ToServer: Sized + IntoIterator<Item = u8> {
    async fn write_to(self, udp_stream: &mut UdpStream) -> io::Result<()> {
        for byte in self {
            udp_stream.write_all(&[byte]).await?;
        }

        Ok(())
    }
}
