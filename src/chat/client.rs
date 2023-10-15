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
                    result = server.subscriber.listen() => {
                        if !server.subscriber.send_to_channel(result, server.socket_addr) {
                            return;
                        }
                    }
                    result = server.producer.subscribe_to_channel() => {
                        if !server.producer.send(result, server.socket_addr).await {
                            return;
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
            let addr = server.socket_addr.clone();
            loop {
                let result = server.subscriber.listen().await;
                if !server.subscriber.send_to_channel(result, addr) {
                    return;
                }
            }
        });

        tokio::spawn(async move {
            match server.producer.send_initial_msg().await {
                Ok(x) => x,
                Err(e) => { println!("Producer init error: {e:?}"); return; }
            };

            loop {
                let result = server.producer.subscribe_to_channel().await;
                if !server.producer.send(result, server.socket_addr.clone()).await {
                   return;
                };
            }
        });
    }
}
