use std::error::Error;
use std::io;
use std::net::SocketAddr;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::broadcast::Sender;

pub(in crate::chat) struct Subscriber {
    pub(in crate::chat) reader: BufReader<OwnedReadHalf>,
    pub(in crate::chat) tx: Sender<(SocketAddr, String)>,
    pub(in crate::chat) buf: String,
}

impl Subscriber {
    pub(in crate::chat) async fn subscribe(
        &mut self,
        result: io::Result<usize>,
        socket_addr: SocketAddr
    ) -> Result<(), Box<dyn Error>> {
        result?;
        self.tx.send((socket_addr, self.buf.clone()))?;
        self.buf.clear();
        Ok(())
    }
}
