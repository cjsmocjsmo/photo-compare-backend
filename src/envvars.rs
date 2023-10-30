use std::env;

pub fn set_env_vars() {
    let compare_pagination = env::var("COMPARE_PAGINATION");
    if compare_pagination.is_err() {
        env::set_var("COMPARE_PAGINATION", "5");
    };
    let compare_image_path = env::var("COMPARE_IMAGE_PATH");
    if compare_image_path.is_err() {
        env::set_var(
            "COMPARE_IMAGE_PATH",
            "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted",
        );
    };
    let compare_json_path = env::var("COMPARE_JSON_PATH");
    if compare_json_path.is_err() {
        env::set_var(
            "COMPARE_JSON_PATH",
            "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove",
        );
    };
    let compare_raw_http = env::var("COMPARE_RAW_HTTP");
    if compare_raw_http.is_err() {
        env::set_var("COMPARE_RAW_HTTP", "192.168.0.91");
    };
    let compare_http = env::var("COMPARE_HTTP_ADDR");
    if compare_http.is_err() {
        env::set_var("COMPARE_HTTP_ADDR", "http://192.168.0.91");
    };
    let compare_port = env::var("COMPARE_PORT");
    if compare_port.is_err() {
        env::set_var("COMPARE_PORT", "8181");
    };
}
