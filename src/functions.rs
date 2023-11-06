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
    pub jsonfilename: String,
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
    let pagination = env::var("COMPARE_PAGINATION")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut files = Vec::new();

    for entry in WalkDir::new(json_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap().to_owned();
            let file_contents = std::fs::read_to_string(file_path).unwrap();
            let img_hash_struct: TransDupsEntry = serde_json::from_str(&file_contents).unwrap();
            files.push(img_hash_struct);

            if files.len() == pagination {
                break;
            }
        }
    }

    files
}

#[get("/completed/{filename}")]
pub async fn completed(f: web::Path<String>) -> impl Responder {
    let prefix = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/";
    let fname = f.into_inner();
    let filename = format!("{}{}", prefix, fname);
    std::fs::remove_file(&filename).unwrap();

    HttpResponse::Ok().body("Single File Deleted!")
}

#[get("/delete_all/{filename}")]
pub async fn delete_all(f: web::Path<String>) -> impl Responder {
    let prefix = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/";
    let fname = f.into_inner();
    let filename = format!("{}{}", prefix, fname);
    println!("Filename: \n\t{}", filename);
    //open filename read it's contents and delete all files
    let file_contents = std::fs::read_to_string(&filename).unwrap();
    let img_hash_struct: TransDupsEntry = serde_json::from_str(&file_contents).unwrap();
    for dup in img_hash_struct.duplicates {
        let prefix2 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
        let file_to_delete = format!("{}{}", prefix2, dup.strdups);
        println!("File to delete: \n\t{}", file_to_delete);
        // std::fs::remove_file(file_to_delete).unwrap();
    }
    // std::fs::remove_file(filename).unwrap();

    HttpResponse::Ok().body("All Deleted!")
}

#[get("/delete_single/{filename}")]
pub async fn delete_single(f: web::Path<String>) -> impl Responder {
    let prefix = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
    let fname = f.into_inner();
    let filename = format!("{}{}", prefix, fname);
    std::fs::remove_file(&filename).unwrap();

    HttpResponse::Ok().body("Single File Deleted!")
}
