use mongodb::{bson::doc, error::Result, options::FindOptions, Collection, Database};

use crate::apis::todos::v1::Todo;

#[derive(Clone, Debug)]
pub struct TodoRepository {
    pub collection: Collection<Todo>,
}

impl TodoRepository {
    pub fn new(database: Database) -> Self {
        Self {
            collection: database.collection("todos"),
        }
    }

    pub async fn create(&self, todo: Todo) -> Result<()> {
        if let Err(error) = self.collection.insert_one(todo.to_owned(), None).await {
            return Err(error);
        }

        return Ok(());
    }

    pub async fn delete_by_id(&self, id: uuid::Uuid) -> Result<Option<()>> {
        let result = self
            .collection
            .delete_one(doc! { "_id": id.simple().to_string()}, None)
            .await?;

        if result.deleted_count == 1 {
            return Ok(Some(()));
        }

        return Ok(None);
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<Option<Todo>> {
        let query = doc! { "_id": id.simple().to_string() };

        self.collection.find_one(query, None).await
    }

    pub async fn list(&self, limit: u64, page: u64) -> Result<Vec<Todo>> {
        let options = FindOptions::builder()
            .skip(limit * page)
            .limit(limit as i64)
            .build();

        let mut cursor = self.collection.find(None, options).await?;

        let mut todos = vec![];

        while cursor.advance().await? {
            let todo = cursor.deserialize_current()?;

            todos.push(todo);
        }

        return Ok(todos);
    }
}
