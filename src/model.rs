use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: i8,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema , IntoParams, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct PeopleModel {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub age: i32,

    #[schema(value_type = String, format = Date)]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct PeopleModelView {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub age: i32,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}