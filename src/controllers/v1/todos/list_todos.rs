use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    apis::{common::ListOptions, todos::v1::ListTodosResponseBody},
    services::v1::TodoService,
};

#[get("/todos")]
pub async fn list_todos(
    query: web::Query<ListOptions>,
    service: web::Data<Mutex<TodoService>>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    if let Ok(todos) = service.lock().unwrap().list_todos(limit, offset).await {
        let todos: ListTodosResponseBody = todos
            .iter()
            .map(|todo| todo.into_get_todo_by_id_response_body())
            .collect();

        if todos.len() == 0 {
            return HttpResponse::NoContent().finish();
        }

        return HttpResponse::Ok().json(todos);
    }

    HttpResponse::InternalServerError().finish()
}
