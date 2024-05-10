use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};

use crate::services::v1::TodoService;

#[get("/todos/{id}")]
pub async fn get_todo_by_id(
    id: web::Path<uuid::Uuid>,
    service: web::Data<Mutex<TodoService>>,
) -> impl Responder {
    if let Ok(todo) = service
        .lock()
        .unwrap()
        .get_todo_by_id(id.into_inner())
        .await
    {
        if todo.is_none() {
            return HttpResponse::NotFound().finish();
        }

        return HttpResponse::Ok().json(todo);
    }

    HttpResponse::InternalServerError().finish()
}
