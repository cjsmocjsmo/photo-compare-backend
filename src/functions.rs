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
pub struct TransDupsEntry {
    pub filename: String,
    pub httpfilename: String,
    pub duplicates: Vec<String>,
    pub httpduplicates: Vec<String>,
}

#[get("/jsonblob")]
pub async fn jsonblob() -> impl Responder {
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();
    let pagination = env::var("COMPARE_PAGINATION").unwrap();

    let dup_info = get_25_files();
    println!("dup_info {:#?}", dup_info.clone());

    // let gen_frag = generate_fragment(dup_info);


    let json = serde_json::to_string(&dup_info).unwrap();

    // println!("Found {} files", json.len());

    HttpResponse::Ok().json(json)
}

// fn generate_fragment(dupslist: Vec<TransDupsEntry>) -> String {

//     for file in dupslist {
//         let mut fragment = Vec::new();
//         let filename = file.clone().httpfilename;
//         let frag1 = format!("<div class='container'><h1>Original</h1>");
//         fragment.push(frag1);
//         let frag2 = format!("<section class='containerImg'>");
//         fragment.push(frag2);
//         let frag3 = format!("<img src={} alt='test1'></section>", file.clone().httpfilename);
//         fragment.push(frag3);
//         let frag4 = format!("<h1>Duplicates</h1><section class='dupImages'>");
//         fragment.push(frag4);
//         for dup in file.duplicates {
//             let frag5 = format!("<div class='dupCard'><img src={} alt='test2'>", dup.clone().httpdups);
//             fragment.push(frag5);
//             let frag6 = format!("<div class='dupCardText'><p>{}</p></div></div>", dup.clone().strdups);
//             fragment.push(frag6);
//         }

//     }

//     "fuck".to_string()
// }

fn get_25_files() -> Vec<TransDupsEntry> {
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();
    let pagination = env::var("COMPARE_PAGINATION").unwrap();

    let mut files = Vec::new();

    for entry in WalkDir::new(json_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap().to_owned();
            // read the json file file_path and parse it into a ImgHashStruct
            let file = std::fs::read_to_string(file_path).unwrap();
            let img_hash_struct: TransDupsEntry = serde_json::from_str(&file).unwrap();
            files.push(img_hash_struct);
            let int_pagination =  pagination.parse::<usize>().unwrap();
            if files.len() == int_pagination {
                break;
            }
        }
    }

    files
}