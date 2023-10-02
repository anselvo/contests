use tokio::io::AsyncBufReadExt;
use crate::chat::server::Server;

pub trait Client {
    fn run(&self, server: Server);
}

pub struct ClientTokioSelect;
impl Client for ClientTokioSelect {
    fn run(&self, mut server: Server) {
        tokio::spawn(async move {
            match server.producer.send_initial_msg().await {
                Ok(x) => x,
                Err(e) => { println!("Producer init error: {e:?}"); return; }
            };

            loop {
                tokio::select! {
                    result = server.subscriber.reader.read_line(&mut server.subscriber.buf) => {
                        match server.subscriber.subscribe(result, server.socket_addr).await {
                            Ok(x) => x,
                            Err(e) => { println!("Subscriber error: {e:?}"); break; }
                        };
                    },
                    result = server.producer.rx.recv() => {
                        match server.producer.produce(result, server.socket_addr).await {
                            Ok(x) => x,
                            Err(e) => { println!("Producer error: {e:?}"); break; }
                        };
                    }
                }
            }
        });
    }
}


pub struct ClientManual;
impl Client for ClientManual {
    fn run(&self, mut server: Server) {
        tokio::spawn(async move {
            loop {
                let result = server.subscriber.reader.read_line(&mut server.subscriber.buf).await;
                match server.subscriber.subscribe(result, server.socket_addr.clone()).await {
                    Ok(x) => x,
                    Err(e) => { println!("Subscriber error: {e:?}"); break; }
                };
            }
        });

        tokio::spawn(async move {
            match server.producer.send_initial_msg().await {
                Ok(x) => x,
                Err(e) => { println!("Producer init error: {e:?}"); return; }
            };

            loop {
                let result = server.producer.rx.recv().await;
                match server.producer.produce(result, server.socket_addr.clone()).await {
                    Ok(x) => x,
                    Err(e) => { println!("Producer error: {e:?}"); break; }
                };
            }
        });
    }
}
