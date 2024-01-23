// use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize};

const SERVICES_TABLE_NAME: &str = "services";

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    // EmptyBranch,
    MysqlError(mysql::Error),
    // Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    // fn status_code(&self) -> StatusCode {
    // match self {
    // PersistenceError::EmptyBranch => StatusCode::BAD_REQUEST,

    // PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
    //     StatusCode::INTERNAL_SERVER_ERROR
    // }
    // }
    // }
}

pub fn insert_service(
    conn: &mut mysql::PooledConn,
    ip: String,
    link: String,
    websocket_key: String,
) -> mysql::error::Result<u64> {
    let insert_query =
        format!("INSERT INTO {SERVICES_TABLE_NAME} (ip, link, websocket_key) VALUES (:ip, :link, :websocket_key)");
    conn.exec_drop(
        insert_query,
        params! {
            "ip"=> ip,
            "link" => link,
            "websocket_key" => websocket_key
        },
    )
    .map(|_| conn.last_insert_id())
}

// pub fn update_service(
//     conn: &mut mysql::PooledConn,
//     link: String,
//     websocket_key: String,
// ) -> mysql::error::Result<u64> {
//     let update_query = format!(
//         "UPDATE {SERVICES_TABLE_NAME} SET link = CONCAT(link, ',{}') WHERE websocket_key = :websocket_key",
//         link
//     );
//     conn.exec_drop(
//         &update_query,
//         params! {
//             "websocket_key" => websocket_key,
//         },
//     )
//     .map(|_| conn.last_insert_id())
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub id: String,
    pub ip: String,
    pub websocket_key: String,
    pub link: String,
}

pub fn get_service(
    conn: &mut mysql::PooledConn,
    search: Option<String>,
) -> Result<Vec<Service>, PersistenceError> {
    let query_fn = |(id, ip, link, websocket_key)| Service {
        id,
        ip,
        link,
        websocket_key,
    };

    match search {
        Some(search) => {
            let select_query = format!("SELECT * FROM {SERVICES_TABLE_NAME} WHERE CONCAT(id, ip, link, websocket_key) LIKE '%{search}%'");
            let data = conn.query_map(select_query, query_fn)?;
            Ok(data)
        }
        None => {
            let select_query = format!("SELECT * FROM {SERVICES_TABLE_NAME}");
            let data = conn.query_map(select_query, query_fn)?;
            Ok(data)
        }
    }
}

pub fn delete_service(
    conn: &mut mysql::PooledConn,
    websocket_key: String,
) -> Result<u64, PersistenceError> {
    let delete_query =
        format!("DELETE FROM {SERVICES_TABLE_NAME} WHERE websocket_key = :websocket_key");
    let data = conn
        .exec_drop(
            delete_query,
            params! {
                "websocket_key" => websocket_key
            },
        )
        .map(|_| conn.last_insert_id())?;
    Ok(data)
}
