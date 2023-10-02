use std::error::Error;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

pub(in crate::chat) struct Producer {
    pub(in crate::chat) writer: OwnedWriteHalf,
    pub(in crate::chat) rx: Receiver<(SocketAddr, String)>,
    pub(in crate::chat) init_identifier_msg: String,
}

impl Producer {
    pub(in crate::chat) async fn produce(
        &mut self,
        result: Result<(SocketAddr, String), RecvError>,
        socket_addr: SocketAddr
    ) -> Result<(), Box<dyn Error>> {
        let (msg_addr, msg) = result?;
        if msg_addr != socket_addr {
            let identifier_msg = Producer::get_identifier_msg(msg_addr, "<");
            let msg = format!("{identifier_msg}{msg}");
            self.writer.write_all(msg.as_bytes()).await?;
        }
        self.writer.write_all(self.init_identifier_msg.as_bytes()).await?;
        Ok(())
    }

    pub(in crate::chat) async fn send_initial_msg(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.write_all(self.get_init_msg().as_bytes()).await?;
        self.writer.write_all(self.init_identifier_msg.as_bytes()).await?;
        Ok(())
    }

    pub(in crate::chat) fn get_identifier_msg(addr: SocketAddr, direction: &str) -> String {
        let port = addr.port();
        let color = (port % (231 - 17)) + 17;
        return format!("\r\u{001b}[38;5;{color}m{port}\u{001b}[0m {direction} ");
    }

    fn get_init_msg(&self) -> &'static str {
        return "Welcome to Dead Coders Society!\n\
        There is only one rule.\n\
        If somebody sees what you're doing,\n\
        you should say this is not Society that you're looking for.\n";
    }
}
