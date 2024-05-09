use std::sync::Mutex;

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

    let todo_service = TodoService::new(todo_repository);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(todo_service.clone())))
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
