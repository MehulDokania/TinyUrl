use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use rand::{Rng};
use rand::distr::Alphanumeric;
use tinyurl_rust::models::*;
use diesel::prelude::*;

use tinyurl_rust::schema::url_map::dsl::*;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Tiny Url Rust Server!")
}

#[post("/shorten")]
async fn shorten_url(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/retrieve")]
async fn retrieve_url(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/count")]
async fn fetch_count(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Utility functions
fn generate_short_url() -> String {
    let random_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    
    format!("https://rust.tiny.url/{}", random_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = &mut tinyurl_rust::establish_connection();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(shorten_url)
            .service(retrieve_url)
            .service(fetch_count)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}