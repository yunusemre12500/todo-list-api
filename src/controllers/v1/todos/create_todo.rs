use std::sync::Mutex;

use actix_web::{http::header, post, web, HttpResponse, Responder};

use crate::{
    apis::todos::v1::{CreateTodoRequestBody, Todo},
    services::v1::TodoService,
};

#[post("/todos")]
pub async fn create_todo(
    body: web::Json<CreateTodoRequestBody>,
    service: web::Data<Mutex<TodoService>>,
) -> impl Responder {
    let new_todo = Todo::from_create_todo_request_body(body.into_inner());

    if let Err(_) = service
        .lock()
        .unwrap()
        .create_todo(new_todo.to_owned())
        .await
    {
        return HttpResponse::InternalServerError().finish();
    }

    return HttpResponse::Created()
        .insert_header((header::CONTENT_LOCATION, new_todo.resource_location()))
        .json(new_todo.into_create_todo_response_body());
}
