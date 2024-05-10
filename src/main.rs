use std::{sync::Mutex, time};

use actix_limitation::{Limiter, RateLimiter};
use actix_web::{web, App, HttpServer};
use repositories::v1::TodoRepository;
use services::v1::TodoService;

mod apis;
mod controllers;
mod database;
mod repositories;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database::init().await;

    let todo_repository = TodoRepository::new(database);

    let todo_service = web::Data::new(Mutex::new(TodoService::new(todo_repository)));

    let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1".to_string());

    let limiter = web::Data::new(
        Limiter::builder(redis_url)
            .limit(120)
            .period(time::Duration::from_secs(60))
            .key_by(|request| Some(request.connection_info().peer_addr().unwrap().to_string()))
            .build()
            .expect("Build limiter failed."),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(RateLimiter::default())
            .app_data(limiter.clone())
            .app_data(todo_service.clone())
            .service(
                web::scope("/v1")
                    .service(controllers::v1::todos::create_todo)
                    .service(controllers::v1::todos::delete_todo_by_id)
                    .service(controllers::v1::todos::get_todo_by_id)
                    .service(controllers::v1::todos::list_todos),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
