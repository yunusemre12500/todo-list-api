use std::sync::Mutex;

use actix_web::{delete, web, HttpResponse, Responder};

use crate::services::v1::TodoService;

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(
    id: web::Path<uuid::Uuid>,
    service: web::Data<Mutex<TodoService>>,
) -> impl Responder {
    if let Ok(result) = service
        .lock()
        .unwrap()
        .delete_todo_by_id(id.into_inner())
        .await
    {
        if result.is_none() {
            return HttpResponse::NotFound().finish();
        }

        return HttpResponse::NoContent().finish();
    }

    HttpResponse::InternalServerError().finish()
}
