use actix_web::{
    get,
    web,
    App,
    HttpServer,
    Responder,
    HttpRequest,
    HttpResponse,
};
use serde::Deserialize;
use actix_web::http::{header, Method, StatusCode};
use actix_session::Session;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

async fn register(req: HttpRequest, form: web::Form<Register>) -> impl Responder {
    println!("{:?}", req);
    format!("Hello {} from {}!", form.username, form.country)
}

#[get("/")]
async fn index(session: Session, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req.match_info());
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html"))
}

async fn getest(session: Session, req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html"))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .route("register", web::get().to(register))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
