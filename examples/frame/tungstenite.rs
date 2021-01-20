use log::{error, info};
use std::collections::HashMap;
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::{
    handshake::server::{Request, Response},
    protocol::Role,
    server::accept_hdr,
    Message, WebSocket,
};

/**
 *  tungstenite: Rust Websocket轻量级框架   
 */
fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let tcp_listener = match TcpListener::bind("0.0.0.0:9000") {
        Ok(listener) => listener,
        Err(err) => {
            error!("bind error: {:#?}", err);
            unreachable!()
        }
    };
    let idx = Arc::new(AtomicU8::new(0));
    let container = Arc::new(Mutex::new(HashMap::<u8, WebSocket<TcpStream>>::new()));
    for stream_result in tcp_listener.incoming() {
        if let Ok(stream) = stream_result {
            if let Ok(peer_addr) = stream.peer_addr() {
                info!("receive a connection from {:#?}", peer_addr);
            }
            let idx_cloned = idx.clone();
            let container_cloned = container.clone();
            thread::spawn(move || {
                // Websocket handshake
                let callback = |req: &Request, response: Response| {
                    info!("The request's headers are:");
                    for (ref _header, _value) in req.headers() {
                        info!("{:#?}: {:#?}", _header, _value);
                    }

                    Ok(response)
                };

                match accept_hdr(stream, callback) {
                    Ok(mut read_websocket) => {
                        let cur_idx = idx_cloned.fetch_add(1, Ordering::SeqCst);
                        // clone stream to create write websocket
                        if let Ok(write_stream) = read_websocket.get_mut().try_clone() {
                            let mut _guard = match container_cloned.lock() {
                                Ok(guard) => guard,
                                Err(poisoned) => poisoned.into_inner(),
                            };
                            let write_websocket =
                                WebSocket::from_raw_socket(write_stream, Role::Server, None);
                            _guard.insert(cur_idx, write_websocket);
                            drop(_guard);
                        }
                        loop {
                            if let Ok(msg) = read_websocket.read_message() {
                                match msg {
                                    Message::Text(text) => {
                                        let mut _guard = match container_cloned.lock() {
                                            Ok(guard) => guard,
                                            Err(poisoned) => poisoned.into_inner(),
                                        };
                                        for (_, write_socket) in _guard.iter_mut() {
                                            let text_cloned = text.clone();
                                            write_socket
                                                .write_message(Message::from(text_cloned))
                                                .unwrap();
                                        }
                                        drop(_guard);
                                    }
                                    Message::Close(_) => {
                                        info!("remove connection idx: {:#?}", cur_idx);
                                        let mut _guard = match container_cloned.lock() {
                                            Ok(guard) => guard,
                                            Err(poisoned) => poisoned.into_inner(),
                                        };
                                        _guard.remove(&cur_idx);
                                        drop(_guard);
                                    }
                                    Message::Ping(_) => {
                                        info!(
                                            "receive ping message connection idx: {:#?}",
                                            cur_idx
                                        );
                                    }
                                    Message::Pong(_) => {
                                        info!(
                                            "receive ping message connection idx: {:#?}",
                                            cur_idx
                                        );
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    Err(err) => {
                        error!("Websocket handshake occur error {:#?}", err);
                    }
                }
            });
        } else {
            error!("tcplistener incoming failed");
        }
    }

    Ok(())
}
