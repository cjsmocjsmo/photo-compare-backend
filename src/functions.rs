use actix_web::{get, web, HttpResponse, Responder};
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

#[get("/jsonblob")]
pub async fn jsonblob() -> impl Responder {
    let dup_info = get_25_files();

    HttpResponse::Ok().json(dup_info)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransDupsEntry {
    pub filename: String,
    pub httpfilename: String,
    pub duplicates: Vec<DupStruct>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DupStruct {
    pub strdups: String,
    pub httpdups: String,
}
fn get_25_files() -> Vec<TransDupsEntry> {
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();
    let pagination = env::var("COMPARE_PAGINATION").unwrap();
    let int_pagination = pagination.parse::<usize>().unwrap();
    let mut files = Vec::new();

    for entry in WalkDir::new(json_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap().to_owned();
            let file_contents = std::fs::read_to_string(file_path).unwrap();
            let img_hash_struct: TransDupsEntry = serde_json::from_str(&file_contents).unwrap();
            files.push(img_hash_struct);

            if files.len() == int_pagination {
                break;
            }
        }
    }

    files
}

#[get("/delete_all/{filename}")]
pub async fn delete_all(f: web::Path<String>) -> impl Responder {
    let filename = f.into_inner();
    std::fs::remove_file(&filename).unwrap();

    HttpResponse::Ok().body("Deleted!")
}
