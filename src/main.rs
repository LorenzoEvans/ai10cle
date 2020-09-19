
//! Example actix-web application.
//!
//! This code is adapted from the front page of the [Actix][] website.
//!
//! [actix]: https://actix.rs/docs/

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use actix_files::NamedFile
use std::sync::Mutex;
use actix::prelude::*;

// An applications scope acts like a namespace for routes.
// All routes for a specific scope, have the same url path prefix.
// Application state is shared with all routes and resources *within the same scope*.

// HttpServer accepts an application factory, as opposed to an instance,
// constructing an instance for each thread.
// You must use a shareable object if you want to share data betwixt threads.

// web:Data uses Arc internally, thus to avoid double Arc, we should create our 
// Data before registering it, using App::app_data()

struct Article {
    title: String,
    content: String
}
struct User {
    username: String,
    password: String,
    liked_articles: Vec<Article>
}
struct AppState {
    // THIS MIGHT BE WHERE WE WANT TO USE OUR LOGIC GATES TO IMPLEMENT THE LS8.
    app_name: String,
    articles: Vec<Article>
    users: Vec<Users>

}

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

// #[derive(Message)]
// #[rtype(result = "usize")]
// This request handler, is an async function,
// that accepts parameters, that can be extracted from a request,
// for example, impl FromRequest, and returns a type that can
// be cast to an HttpResponse, impl Responder.


async home(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Welcome to {}!", app_name)
}

async fn articles() -> impl Responder {
    HttpResponse::Ok().body("Here are the articles!")
}

async fn profile() -> impl Responder {
    HttpResponse::Ok().body("Here is your profile")
}
// Next we have an instance of App from actix-web,
// and we register the request handlers, with the applications,
// route on a path, with a particular Http method.
// This returns an Application factory.
    //  Check out Factory Pattern
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("AI10cle")
    });
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/articles", web::get().to(index))
            .route("/profile", web::get().to(index_2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
        
}

