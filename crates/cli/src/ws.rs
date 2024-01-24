use colored::Colorize;
use serde::{Deserialize, Serialize};
use websockets::{Frame, WebSocket};

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    pub ip: String,
    pub link: String,
}

pub async fn connect_and_handle_messages(service: Service) {
    println!("Ws: server connecting...");
    match WebSocket::connect("ws://120.55.189.199:8080/ws").await {
        Ok(mut ws) => {
            println!("{}", "Ws: server connect success".green());
            let value = serde_json::to_string(&service).unwrap();
            ws.send_text(value).await.unwrap();

            loop {
                let msg = ws.receive().await;
                match msg {
                    Ok(msg) => {
                        match msg {
                            Frame::Text { payload, .. } => {
                                // 处理接收到的文本消息
                                println!("➤   Received: {}", payload);
                            }
                            Frame::Close { .. } => {
                                println!("➤   Received close");
                                break;
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        println!("websocket connect fail: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            println!("websocket connect fail: {}", err);
        }
    };
}
