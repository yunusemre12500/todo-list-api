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
    if let Ok(todos) = service
        .lock()
        .unwrap()
        .list_todos(query.limit.unwrap_or(100), query.page.unwrap_or(0))
        .await
    {
        let todos: ListTodosResponseBody = todos
            .iter()
            .map(|todo| todo.into_get_todo_by_id_response_body())
            .collect();

        return HttpResponse::Ok().json(todos);
    }

    HttpResponse::InternalServerError().finish()
}
