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
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::http::{Method, StatusCode};
use actix_web::{
    guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod model;
mod router;
mod schema;
mod share;

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

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("http://localhost:8080")
            //         .allowed_methods(vec!["GET", "POST"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .max_age(3600),
            // )
            .wrap(middleware::Logger::default())
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
            //         .name("auth")
            //         .path("/")
            //         .domain(domain.as_str())
            //         .max_age_time(chrono::Duration::days(1))
            //         .secure(false), // set true if https
            // ))
            .service(favicon)
            .service(welcome)
            .configure(router::lots)
            .configure(router::users)
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
