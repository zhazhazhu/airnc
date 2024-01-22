use serde::{Deserialize, Serialize};
use std::process;
use tokio::{
    select,
    signal::unix::{signal, SignalKind},
};
use websockets::{Frame, WebSocket};

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    pub ip: String,
    pub link: String,
}

pub async fn connect_and_handle_messages(service: Service) {
    match WebSocket::connect("ws://127.0.0.1:8080/ws").await {
        Ok(mut ws) => {
            let value = serde_json::to_string(&service).unwrap();
            ws.send_text(value).await.unwrap();
            let mut signal = signal(SignalKind::interrupt()).unwrap();

            loop {
                select! {
                    _ = signal.recv() => {
                        process::exit(1);
                    }
                    msg = ws.receive() => {
                        match msg {
                            Ok(msg) => {
                                match msg {
                                    Frame::Text { payload, .. } => {
                                        // 处理接收到的文本消息
                                        println!("Received text message: {}", payload);
                                    }
                                    Frame::Close { .. } => {
                                        println!("Received close message");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Err(err) => {
                                println!("websocket connect fail: {}", err);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("websocket connect fail: {}", err);
        }
    };
}
