use mongodb::{Client, Database};

pub async fn init() -> Database {
    let uri = std::env::var("DATABASE_URI").unwrap_or("mongodb://127.0.0.1".to_string());

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect database.");

    client.database("todo-app")
}
