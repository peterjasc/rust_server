use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
}