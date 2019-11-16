#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[allow(dead_code)]
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

extern crate serde;
extern crate serde_json;

use actix_files as fs;
use std::{env, io};
// use actix_session::{CookieSession, Session};
//use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod model;
mod router;
mod schema;
mod share;

// ============================
#[get("/favicon")]
fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[get("/")]
fn welcome(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

// ========================

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("e-tron");

    // Start 3 db executor actors
    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // let schema = std::sync::Arc::new(create_schema());

    //let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

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
            .configure(router::users)
            .configure(router::user)
            .configure(router::login)
            .configure(router::factories)
            .configure(router::buy_factories)
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
