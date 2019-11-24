use actix_web::{HttpRequest, HttpResponse};
use chrono;
use chrono::prelude::*;
use std::convert::TryInto;

///chrono::Local::now().naive_utc().timestamp()
pub fn get_time_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{:?}", chrono::Utc::now().naive_utc()))
}

pub fn get_current_time_diff(old_time: NaiveDateTime) -> i32 {
    let now = chrono::Utc::now().naive_utc().timestamp();
    let diff: i32 = ((now - old_time.timestamp()) / 36000).try_into().unwrap();

    match diff <= 0 {
        true => return 1,
        false => return diff,
    }
}
