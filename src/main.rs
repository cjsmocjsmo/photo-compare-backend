use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;

pub mod envvars;
pub mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let img_path = env::var("COMPARE_IMAGE_PATH").unwrap();
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();

    let socket = functions::gen_server_addr();

    HttpServer::new(move || {
        App::new()
            .service(functions::test)
            // .service(crate::functions::echo)


            // .service(crate::server::server_functions::voyager)
            // .service(crate::server::server_functions::wheeloftime)
            .service(fs::Files::new("/json", json_path.clone()).show_files_listing())
            .service(fs::Files::new("/image", img_path.clone()).show_files_listing())
        }
    )
    // .bind(("192.168.0.26", 8080))?
    .bind(socket)?
    .run()
    .await
}
