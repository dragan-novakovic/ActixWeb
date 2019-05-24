#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate serde;
extern crate serde_json;

use actix_files as fs;
use std::{env, io};
// use actix_session::{CookieSession, Session};
use actix_web::http::{Method, StatusCode};
use actix_web::{
    guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
// use bytes::Bytes;
// use futures::unsync::mpsc;
use futures::{future::ok, Future};

mod api;
mod model;
mod router;
mod schema;

/// favicon handler
#[get("/favicon")]
fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[get("/")]
fn welcome(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// async handler
fn index_async(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap())))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("basic-example");

    // Start 3 db executor actors
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://dragan1810:123@localhost/lolspot");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(favicon)
            .service(welcome)
            .configure(router::lots)
            .service(web::resource("/async/{name}").route(web::get().to_async(index_async)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // default
            .default_service(
                web::resource("").route(web::get().to(p404)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(|| HttpResponse::MethodNotAllowed()),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}

/*

/// async body
fn index_async_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));

    HttpResponse::Ok()
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

/// handler with path parameters like `/user/{name}/`
fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}




 */
