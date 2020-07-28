// QUERYSTRING

use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}


// JSON 
//
use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

//use actix_web::{error, web, FromRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(web::Json::<Info>::configure(|cfg| {
                    cfg.limit(4096).error_handler(|err, _req| {
                        // create custom error response
                        error::InternalError::from_response(
                            err,
                            HttpResponse::Conflict().finish(),
                        )
                        .into()
                    })
                }))
                .route(web::post().to(index)),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}


//
//

let rows_inserted = diesel::insert_into(users)
    .values(&name.eq("Sean"))
    .execute(&connection);

assert_eq!(Ok(1), rows_inserted);

let new_users = vec![
    name.eq("Tess"),
    name.eq("Jim"),
];

let rows_inserted = diesel::insert_into(users)
    .values(&new_users)
    .execute(&connection);

assert_eq!(Ok(2), rows_inserted);

//

let updated_row = diesel::update(users.filter(id.eq(1)))
    .set(name.eq("James"))
    .get_result(&connection);
// On backends that support it, you can call `get_result` instead of `execute`
// to have `RETURNING *` automatically appended to the query. Alternatively, you
// can explicitly return an expression by using the `returning` method before
// getting the result.
assert_eq!(Ok((1, "James".to_string())), updated_row);
