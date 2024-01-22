use std::{
    error::Error,
    io::stdout,
    net::{IpAddr, TcpListener},
};

use image::{DynamicImage, Rgba};
use qrcode::QrCode;
use rand::Rng;
use termimage::ops;

pub fn get_local_ip() -> Option<IpAddr> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let ip = socket.local_addr().ok()?.ip();
    Some(ip)
}

pub fn find_available_port() -> u16 {
    let mut rng = rand::thread_rng();
    loop {
        let port: u16 = rng.gen_range(10000..65535);
        if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
            drop(listener);
            return port;
        }
    }
}

pub fn create_qrcode(content: &str) -> Result<(), Box<dyn Error>> {
    let code = QrCode::new(content).unwrap();
    let light = Rgba([236, 236, 236, 1]);
    let dark = Rgba([0, 0, 0, 1]);

    let image = code
        .render::<Rgba<u8>>()
        .max_dimensions(1, 1)
        .light_color(light)
        .dark_color(dark)
        .build();

    let img = DynamicImage::ImageRgba8(image);
    ops::write_ansi_truecolor(&mut stdout(), &img);
    Ok(())
}
