// #[cfg(test)]
// mod tests {
//     use actix_web::{http, test, web, Error, HttpResponse};

//     #[actix_rt::test]
//     async fn test_index() {
//         let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();

//         let resp = factories(req).await.unwrap();
//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let req = test::TestRequest::default().to_http_request();
//         let resp = factories(req).await.unwrap();
//         assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
//     }
// }

// #[actix_rt::test]
// async fn test_request_response_form() {
//     let mut app = init_service(App::new().service(web::resource("/people").route(
//         web::post().to(|person: web::Form<Person>| {
//             async { HttpResponse::Ok().json(person.into_inner()) }
//         }),
//     )))
//     .await;

//     let payload = Person {
//         id: "12345".to_string(),
//         name: "User name".to_string(),
//     };

//     let req = TestRequest::post()
//         .uri("/people")
//         .set_form(&payload)
//         .to_request();

//     assert_eq!(req.content_type(), "application/x-www-form-urlencoded");

//     let result: Person = read_response_json(&mut app, req).await;
//     assert_eq!(&result.id, "12345");
//     assert_eq!(&result.name, "User name");
// }
