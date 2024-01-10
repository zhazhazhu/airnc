use crate::{
    cli::Cli,
    utils::{create_qrcode, find_available_port, get_local_ip},
};
use actix_web::{
    http::header::ContentLength,
    web::{self, Bytes},
    App, HttpResponse, HttpServer, Responder,
};
use async_stream::stream;
use std::{
    fs::File,
    io::{self, Read},
    net::SocketAddr,
    path::PathBuf,
    process::exit,
};
use tokio::sync::mpsc;

#[derive(Clone)]
struct AppState {
    file_path: PathBuf,
}

#[tokio::main]
pub async fn run_server(cli: Cli) -> Result<(), std::io::Error> {
    let file_path = PathBuf::from(cli.path.unwrap());
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    if !file_path.exists() {
        eprintln!("File not found: {:?}", file_path);
        exit(1);
    }

    let state = AppState { file_path };

    let local_ip = get_local_ip().unwrap();

    let port = find_available_port();

    let addr = SocketAddr::new(local_ip, port);

    let download_url = format!("http://{}/{}", addr.to_string(), &file_name);

    create_qrcode(&download_url).unwrap();

    println!("\nDownload URL: {}", download_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route(&format!("/{}", file_name), web::get().to(download_handler))
    })
    .bind(addr)?
    .run()
    .await
}

async fn download_handler(data: web::Data<AppState>) -> io::Result<impl Responder> {
    let mut file = File::open(&data.file_path)?;
    let file_size = file.metadata()?.len();
    let (sender, mut receiver) = mpsc::channel::<Result<Bytes, io::Error>>(32);

    let download_task = tokio::task::spawn_blocking(move || {
        let mut buffer = [0u8; 8192];
        let mut remaining_bytes = file_size;

        while remaining_bytes > 0 {
            let bytes_to_read = buffer.len().min(remaining_bytes as usize);
            let bytes_read = file.read(&mut buffer[..bytes_to_read])?;

            let chunk = Bytes::copy_from_slice(&buffer[..bytes_read]);
            sender
                .blocking_send(Ok(chunk))
                .expect("Failed to send chunk");

            remaining_bytes -= bytes_read as u64;
        }

        Ok::<_, io::Error>(())
    });

    tokio::spawn(async move {
        if let Err(e) = download_task.await {
            eprintln!("Error occurred during file download: {:?}", e);
        }
    });

    let s = stream! {
        while let Some(chunk) = receiver.recv().await {
            yield chunk.map_err(actix_web::Error::from);
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header(ContentLength(file_size as usize))
        .streaming(s))
}
