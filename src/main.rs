use std::io::Result;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn greet(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("Hello"))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = reqr::run();

    if let Err(err) = args {
        eprintln!("{}", err);
        std::process::exit(1)
    }

    let url = args?.url;
    println!("Succeed! Url: {}", url);

    HttpServer::new(move || {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
