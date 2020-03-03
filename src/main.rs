use actix_files as fs;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
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

#[derive(Template)]
#[template(path = "input.html")]
struct Input {}

#[derive(Template)]
#[template(path = "result.html")]
struct PasteResult<'a> {
    url: &'a str,
    id: &'a str,
    content: &'a str,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(favicon)
            .service(fs::Files::new("/static", "public/static/").show_files_listing())
            .service(index)
            .service(paste_handler)
            .service(query_handler)
            .service(raw_handler)
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
    PasteResult {
        url: &req
            .connection_info()
            .let_imut(|ctx| format!("{}://{}", ctx.scheme(), ctx.host())),
        id: &req.uri().to_string(),
        content: &get_paste(&id),
    }
    .let_imut(|paste_result| {
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(paste_result.render().unwrap())
    })
}

#[get("/raw/{id}")]
async fn raw_handler(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    println!("{:?}", id);
    get_paste(&id)
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
        .body(Input {}.render().unwrap())
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
