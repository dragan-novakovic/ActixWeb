// //auth_handler.rs
// use actix_identity::Identity;
// use actix_web::{
//     dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest, HttpResponse,
// };
// use diesel::prelude::*;
// use futures::Future;

// use crate::model::user::{NewUser, User};
// use crate::share::db::Pool;

// #[derive(Debug, Deserialize)]
// pub struct AuthData {
//     pub email: String,
//     pub password: String,
// }

// // we need the same data
// // simple aliasing makes the intentions clear and its more readable
// pub type LoggedUser = NewUser;

// // to get LoggedUser from auth cookie
// impl FromRequest for LoggedUser {
//     type Error = Error;
//     type Future = Result<LoggedUser, Error>;
//     type Config = ();

//     fn from_request(req: &HttpRequest, pl: &mut Payload) -> Result<LoggedUser, Error> {
//         if let Some(identity) = Identity::from_request(req, pl)?.identity() {
//             let user: LoggedUser = serde_json::from_str(&identity)?;
//             return Ok(user);
//         }
//         Err(Error::from(HttpResponse::Unauthorized().json("Un")))
//     }
// }

// pub fn logout(id: Identity) -> HttpResponse {
//     id.forget();
//     HttpResponse::Ok().finish()
// }

// pub fn login(
//     auth_data: web::Json<NewUser>,
//     id: Identity,
//     pool: web::Data<Pool>,
// ) -> impl Future<Item = HttpResponse, Error> {
//     web::block(move || query(auth_data.into_inner(), pool)).then(
//         move |res: Result<NewUser, BlockingError>| match res {
//             Ok(user) => {
//                 let user_string = serde_json::to_string(&user).unwrap();
//                 id.remember(user_string);
//                 Ok(HttpResponse::Ok().finish())
//             }
//             Err(err) => match err {
//                 BlockingError::Error(service_error) => Err(service_error),
//                 BlockingError::Canceled => Err(ServiceError::InternalServerError),
//             },
//         },
//     )
// }

// pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
//     HttpResponse::Ok().json(logged_user)
// }
// /// Diesel query
// fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<NewUser, Error> {
//     use crate::schema::users::dsl::{email, users};
//     let conn: &PgConnection = &pool.get().unwrap();
//     let mut items = users
//         .filter(email.eq(&auth_data.email))
//         .load::<User>(conn)?;

//     if let Some(user) = items.pop() {
//         if let Ok(matching) {
//             user.into()
//         }
//     }
//     Err(ServiceError::Unauthorized)
// }
