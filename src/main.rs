// use axum::{routing::get, Router};
// use config::database::DatabaseRepository;
// use dotenv::dotenv;
// use std::env;

#[macro_use]
extern crate rocket;
use modules::user::models::user_model::User;
use rocket::http::Method;
use rocket::Route;

mod config;
mod modules;
use modules::user::user_handler::UserHandler;

#[get("/")]
fn index() -> &'static str {
    "¡Hola, mundo!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ping])
}

// #[tokio::main]
// async fn main() {
//     dotenv().ok();

//     // DatabaseRepository::new();

//     let port: String = env::var("PORT").expect("Environment variable 'PORT' not defined.");

//     // build our application with a route
//     let router = Router::new()
//         // `GET /` goes to `root`
//         .route("/", get(root));

//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
//         .await
//         .unwrap();
//     axum::serve(listener, router).await.unwrap();
// }

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }
