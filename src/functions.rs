use actix_web::{get, HttpResponse, Responder};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

pub fn gen_server_addr() -> SocketAddr {
    let raw_addr = env::var("COMPARE_RAW_ADDR").expect("COMPARE_RAW_ADDR not set");
    let compare_v4_addr = Ipv4Addr::from_str(&raw_addr).unwrap();
    let port: u16 = env::var("COMPARE_SERVER_PORT")
        .expect("COMPARE_SERVER_PORT not set")
        .parse()
        .unwrap();
    let socket = SocketAddr::new(IpAddr::V4(compare_v4_addr), port);

    socket
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}
