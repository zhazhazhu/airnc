use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;
use serde::{Deserialize, Serialize};

use crate::persistence::{delete_service, get_service, insert_service};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub ip: String,
    pub link: String,
}

pub struct WsConn {
    pub ip: String,
    pub data: web::Data<mysql::Pool>,
    pub websocket_key: String,
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(self.websocket_key.to_string());
        println!("{} join!", self.websocket_key);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut conn = self.data.get_conn().unwrap();
        let websocket_key = self.websocket_key.clone();
        delete_service(&mut conn, websocket_key).unwrap();
        println!("{} exit!", self.websocket_key);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match message {
            Ok(Message::Text(msg)) => {
                let pool = self.data.clone();
                let websocket_key = self.websocket_key.clone();
                let data = serde_json::from_str::<Data>(&msg).unwrap();

                tokio::spawn(async {
                    web::block(move || {
                        let mut conn = pool.get_conn().unwrap();
                        let id = insert_service(&mut conn, data.ip, data.link, websocket_key)
                            .expect("insert service error");
                        Some(id)
                    })
                    .await
                    .unwrap();
                });
            }
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let ip = req.peer_addr().expect("Unknown Ip").ip().to_string();
    let websocket_key = req
        .headers()
        .get("sec-websocket-key")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let conn = WsConn {
        ip,
        data,
        websocket_key,
    };
    let resp = ws::start(conn, &req, stream);
    resp
}

#[derive(Deserialize)]
pub struct ClientQuery {
    pub search: Option<String>,
}

#[get("/client")]
pub async fn clients(
    query: web::Query<ClientQuery>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = data.get_conn().unwrap();
    let services = web::block(move || get_service(&mut conn, query.search.clone())).await??;
    Ok(web::Json(services))
}
