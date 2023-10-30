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
    pub duplicates: Vec<DupStruct>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DupStruct {
    pub strdups: String,
    pub httpdups: String,
}

#[get("/jsonblob")]
pub async fn jsonblob() -> impl Responder {

    let dup_info = get_25_files();
    // for dup in dup_info.clone() {
    //     for d in dup.duplicates.clone() {
    //         println!("d {:#?}", d.httpdups);
    //     }
    // }


    let gen_frag = generate_fragment(dup_info);

    println!("gen_frag {:#?}", gen_frag.clone());

    let json = serde_json::to_string(&gen_frag).unwrap();

    // println!("Found {} files", json.len());

    HttpResponse::Ok().json(json)
}

fn generate_fragment(dupslist: Vec<TransDupsEntry>) -> String {
    let mut new_dups = Vec::new();
    for file in dupslist {
        let mut fragment = Vec::new();
        // let filename = file.clone().httpfilename;
        let frag1 = format!("<div class='container'><h1>Original</h1>");
        fragment.push(frag1);
        let frag2 = format!("<section class='containerImg'>");
        fragment.push(frag2);
        let frag3 = format!(
            "<img src={} alt='test1'></section>",
            file.clone().httpfilename
        );
        fragment.push(frag3);
        let frag4 = format!("<h1>Duplicates</h1><section class='dupImages'>");
        fragment.push(frag4);
        for dup in file.duplicates {
            let frag5 = format!(
                "<div class='dupCard'><img src={} alt='test2'>",
                dup.clone().httpdups
            );
            fragment.push(frag5);
            let frag6 = format!(
                "<button on:click={}>delete</button></div>",
                dup.clone().strdups
            );
            fragment.push(frag6);
        }
        let frag7 = format!("</section><div class='completeBtn'>");
        fragment.push(frag7);
        let frag8 = format!("<button class='completeBtn'>Complete</button></div></div>");
        fragment.push(frag8);
        let html = fragment.join("");
        println!("html {:#?}", html.clone());
        new_dups.push(html);
    }

    new_dups.join("")
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
            // read the json file file_path and parse it into a ImgHashStruct
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
