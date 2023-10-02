use std::net::SocketAddr;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use crate::chat::client::Client;
use crate::chat::producer::Producer;
use crate::chat::subscriber::Subscriber;

pub struct Server {
    pub(in crate::chat) subscriber: Subscriber,
    pub(in crate::chat) producer: Producer,
    pub(in crate::chat) socket_addr: SocketAddr,
}

impl Server {
    pub async fn run<T: Client>(host: &str, port: usize, client: T) {
        println!("Starting a chat!");
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr).await.unwrap();
        let (tx, _) = broadcast::channel(10);

        loop {
            let (socket, socket_addr) = match listener.accept().await {
                Ok((s, a)) => {
                    println!("New client: {a:?}");
                    (s, a)
                }
                Err(e) => {
                    println!("New client: {e:?}");
                    break;
                }
            };
            let (reader, writer) = socket.into_split();

            let subscriber = Subscriber {
                reader: BufReader::new(reader),
                tx: tx.clone(),
                buf: String::new(),
            };

            let producer = Producer {
                writer,
                rx: tx.subscribe(),
                init_identifier_msg: Producer::get_identifier_msg(socket_addr, ">"),
            };

            client.run(Server { subscriber, producer, socket_addr });
        }
    }
}
