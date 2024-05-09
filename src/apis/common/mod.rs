use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ListOptions {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}
