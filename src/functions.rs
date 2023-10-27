use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use walkdir::WalkDir;

pub fn gen_server_addr() -> SocketAddr {
    let raw_addr = env::var("COMPARE_RAW_HTTP").expect("COMPARE_RAW_HTTP not set");
    let compare_v4_addr = Ipv4Addr::from_str(&raw_addr).unwrap();
    let port: u16 = env::var("COMPARE_PORT")
        .expect("COMPARE_PORT not set")
        .parse()
        .unwrap();
    let socket = SocketAddr::new(IpAddr::V4(compare_v4_addr), port);

    socket
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DupsStruct {
    pub filename: String,
    pub duplicates: Vec<String>,
}

#[get("/jsonblob")]
pub async fn jsonblob() -> impl Responder {
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();

    let mut files = Vec::new();

    for entry in WalkDir::new(json_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap().to_owned();
            // read the json file file_path and parse it into a ImgHashStruct
            let file = std::fs::read_to_string(file_path).unwrap();
            let img_hash_struct: DupsStruct = serde_json::from_str(&file).unwrap();
            files.push(img_hash_struct);
            if files.len() == 100 {
                break;
            }
        }
    }

    let json = serde_json::to_string(&files).unwrap();

    println!("Found {} files", files.len());

    HttpResponse::Ok().json(json)
}
