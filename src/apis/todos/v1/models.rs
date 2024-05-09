use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
}

impl Todo {
    pub fn from_create_todo_request_body(body: CreateTodoRequestBody) -> Self {
        Self {
            created_at: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            id: uuid::Uuid::new_v4().simple().to_string(),
            title: body.title,
        }
    }

    pub fn into_create_todo_response_body(&self) -> CreateTodoResponseBody {
        CreateTodoResponseBody {
            created_at: self.created_at.to_owned(),
            id: self.id.to_owned(),
            title: self.title.to_owned(),
        }
    }

    pub fn into_get_todo_by_id_response_body(&self) -> GetTodoByIdResponseBody {
        GetTodoByIdResponseBody {
            created_at: self.created_at.to_owned(),
            id: self.id.to_owned(),
            title: self.title.to_owned(),
        }
    }

    pub fn resource_location(&self) -> String {
        format!("/v1/todos/{}", self.id)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTodoRequestBody {
    pub title: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateTodoResponseBody {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetTodoByIdResponseBody {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub id: String,
    pub title: String,
}

pub type ListTodosResponseBody = Vec<GetTodoByIdResponseBody>;
