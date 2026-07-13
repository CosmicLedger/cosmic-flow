pub mod routes;
pub mod handlers;

use actix_cors::Cors;
use actix_web::http;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().ends_with(b"cosmicledger.io") 
                || origin.as_bytes().starts_with(b"http://localhost")
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
