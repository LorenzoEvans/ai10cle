use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .route("/healthcheck",
                web::get().to(health_check)))
                    .bind("127.0.0.1.8000")?
                    .run()
                    .await
}


// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let state = web::Data::new(AppState {
//         app_name: String::from("AI10cle"),
//     });
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(home))
//             .route("/articles", web::get().to(articles))
//             .route("/profile", web::get().to(profile))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
        
// }