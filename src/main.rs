use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;

pub mod envvars;
pub mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _vars = envvars::set_env_vars();
    let img_path = env::var("COMPARE_IMAGE_PATH").unwrap();
    let json_path = env::var("COMPARE_JSON_PATH").unwrap();
    let socket = functions::gen_server_addr();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(functions::test)
            .service(fs::Files::new("/json", json_path.clone()).show_files_listing())
            .service(fs::Files::new("/image", img_path.clone()).show_files_listing())
    })
    .bind(socket)?
    .run()
    .await
}
