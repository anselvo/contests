use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::broadcast::Sender;
use crate::chat::producer::Message;

pub(in crate::chat) struct Subscriber {
    reader: BufReader<OwnedReadHalf>,
    buf: String,
    tx: Sender<Message>,
}

impl Subscriber {
    pub(in crate::chat) fn new(reader: OwnedReadHalf, tx: Sender<Message>) -> Self {
        return Subscriber { reader: BufReader::new(reader), buf: String::new(), tx };
    }

    pub(in crate::chat) async fn listen(&mut self) -> io::Result<usize> {
        return self.reader.read_line(&mut self.buf).await;
    }

    pub(in crate::chat) fn send_to_channel(
        &mut self,
        result: io::Result<usize>,
        socket_addr: SocketAddr
    ) -> bool {
        match result {
            Ok(r) => if r != 0 {
                self.send_with_error_log(socket_addr, self.buf.clone(), false);
            } else {
                println!("{socket_addr} - connection was closed");
                return false;
            },
            Err(e) => {
                self.send_with_error_log(socket_addr, String::from("cannot parse message"), true);
                println!("Subscriber error: cannot parse message -- {e:?}");
            }
        };
        self.buf.clear();
        return true;
    }
    fn send_with_error_log(&mut self, socket_addr: SocketAddr, message: String, error: bool) {
        match self.tx.send(Message { socket_addr, message, error }) {
            Ok(_) => {}
            Err(e) => {
                println!("Subscriber error: cannot send message to channel -- {e:?} {e}");
            }
        };
    }
}
