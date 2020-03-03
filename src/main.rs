use actix_files as fs;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::{distributions::Alphanumeric, Rng};
use redis::{Commands, Connection};
use serde::Deserialize;
use std::sync::Mutex;
use utils::kt_std::*;

mod builder;
mod utils;

#[macro_use]
extern crate lazy_static;

#[derive(Deserialize)]
struct Paste {
    content: String,
}

lazy_static! {
    static ref REDIS_CLIENT: Mutex<Connection> =
        Mutex::new(builder::redis_client::build("redis://127.0.0.1/"));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(favicon)
            .service(paste_handler)
            .service(query_handler)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/favicon.ico")]
async fn favicon() -> fs::NamedFile {
    fs::NamedFile::open("public/static/favicon.ico").unwrap()
}

#[get("/{id}")]
async fn query_handler(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    println!("{:?}", id);
    match id.len() {
        5 => get_paste(&id),
        _ => format!("Nah"),
    }
}

#[post("/paste")]
async fn paste_handler(req: HttpRequest, paste: web::Form<Paste>) -> impl Responder {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .collect::<String>()
        .let_imut(|key| {
            set_paste(&key, &paste);
            HttpResponse::Found()
                .header(http::header::LOCATION, format!("/{}", key))
                .finish()
        })
}

#[get("/")]
async fn index(session: Session, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req.match_info());
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html"))
}

fn set_paste(key: &String, paste: &web::Form<Paste>) -> redis::RedisResult<()> {
    let _: () = REDIS_CLIENT.lock().unwrap().set(key, &paste.content)?;
    Ok(())
}

fn get_paste(key: &String) -> String {
    REDIS_CLIENT
        .lock()
        .unwrap()
        .get(key)
        .unwrap_or("None".to_string())
}
