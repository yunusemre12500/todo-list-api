use crate::{apis::todos::v1::Todo, repositories::v1::TodoRepository};
use mongodb::error::Result;

#[derive(Clone, Debug)]
pub struct TodoService {
    repository: TodoRepository,
}

impl TodoService {
    pub fn new(repository: TodoRepository) -> Self {
        Self { repository }
    }

    pub async fn create_todo(&self, todo: Todo) -> Result<()> {
        self.repository.create(todo).await
    }

    pub async fn delete_todo_by_id(&self, id: uuid::Uuid) -> Result<Option<()>> {
        self.repository.delete_by_id(id).await
    }

    pub async fn get_todo_by_id(&self, id: uuid::Uuid) -> Result<Option<Todo>> {
        self.repository.get_by_id(id).await
    }

    pub async fn list_todos(&self, limit: u64, page: u64) -> Result<Vec<Todo>> {
        self.repository.list(limit, page).await
    }
}
