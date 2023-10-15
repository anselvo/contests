use std::error::Error;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

#[derive(Clone, Debug)]
pub(in crate::chat) struct Message {
    pub(in crate::chat) socket_addr: SocketAddr,
    pub(in crate::chat) message: String,
    pub(in crate::chat) error: bool,
}

pub(in crate::chat) struct Producer {
    rx: Receiver<Message>,
    writer: OwnedWriteHalf,
    init_identifier_msg: String,
}

impl Producer {
    pub(in crate::chat) fn new(writer: OwnedWriteHalf, rx: Receiver<Message>, socket_addr: SocketAddr) -> Self {
        let init_identifier_msg = Self::get_identifier_msg(socket_addr, String::from(">"));
        return Producer { writer, rx, init_identifier_msg };
    }

    pub(in crate::chat) async fn subscribe_to_channel(&mut self) -> Result<Message, RecvError> {
        return self.rx.recv().await;
    }

    pub(in crate::chat) async fn send(
        &mut self,
        result: Result<Message, RecvError>,
        socket_addr: SocketAddr
    ) -> bool {
        let m = match result {
            Ok(m) => { m }
            Err(e) => {
                println!("Producer error: {e}");
                return false;
            }
        };

        if m.socket_addr != socket_addr && !m.error {
            let msg = m.message;
            let identifier_msg = Self::get_identifier_msg(m.socket_addr, String::from("<"));
            let msg = format!("{identifier_msg}{msg}");
            self.writer.write_all(msg.as_bytes()).await.unwrap_or_else(|e| println!("{socket_addr} {e}"));
        } else if m.socket_addr == socket_addr && m.error {
            self.writer.shutdown().await.unwrap();
            return false;
        }

        self.writer.write_all(self.init_identifier_msg.as_bytes()).await.unwrap_or_else(|e| println!("{socket_addr} {e}"));
        return true;
    }

    pub(in crate::chat) async fn send_initial_msg(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.write_all(self.get_init_msg().as_bytes()).await?;
        self.writer.write_all(self.init_identifier_msg.as_bytes()).await?;
        return Ok(())
    }

    fn get_identifier_msg(addr: SocketAddr, direction: String) -> String {
        let port = addr.port();
        let color = (port % (231 - 17)) + 17;
        return format!("\r\u{001b}[38;5;{color}m{addr}\u{001b}[0m {direction} ");
    }

    fn get_init_msg(&self) -> &'static str {
        return "Welcome to Dead Coders Society!\n\
        There is only one rule.\n\
        If somebody sees what you're doing,\n\
        you should say this is not Society that you're looking for.\n";
    }
}
