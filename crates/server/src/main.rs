mod persistence;
mod routes;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use routes::{clients, ws_index};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let db_port = db_port.parse().unwrap();

    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = mysql::Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);

    println!("hello airnc server");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, req_head| {
                println!("{:?}", origin);
                origin == req_head.headers().get("origin").unwrap()
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(shared_data.clone())
            .route("/ws", web::get().to(ws_index))
            .service(web::scope("/api").service(clients))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
}
