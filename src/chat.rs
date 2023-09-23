use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

pub async fn run(host: &str, port: usize) {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr).await.unwrap();
    let (tx, _) = broadcast::channel(10);

    loop {
        let (mut socket, soc_addr) = match listener.accept().await {
            Ok((s, a)) => { println!("New client: {a:?}"); (s, a) },
            Err(e) => {
                println!("New client: {e:?}");
                break;
            },
        };
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut buf = String::new();

            let init_identifier_msg = get_identifier_msg(soc_addr, ">");

            loop {
                tokio::select! {
                    result = {
                        writer.write_all(init_identifier_msg.as_bytes()).await.unwrap();
                        reader.read_line(&mut buf)
                    } => {
                        match result {
                            Ok(_) => {},
                            Err(e) => { println!("{e:?}"); break; },
                        }
                        let msg = buf.clone();
                        tx.send((soc_addr, msg)).unwrap();
                        buf.clear();
                    }
                    result = rx.recv() => {
                        let (msg_addr, msg) = result.unwrap();
                        if msg_addr != soc_addr {
                            let identifier_msg = get_identifier_msg(msg_addr, "<");
                            let msg = format!("{identifier_msg}{msg}");
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

fn get_identifier_msg(addr: SocketAddr, direction: &str) -> String {
    let port = addr.port();
    let color = (port % (231 - 17)) + 17;
    return format!("\r\u{001b}[38;5;{color}m{port}\u{001b}[0m {direction} ");
}
