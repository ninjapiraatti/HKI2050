use super::super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tags"]
pub struct Tag {
  pub id: uuid::Uuid,
  pub title: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "contenttags"]
pub struct ContentTag {
  pub id: uuid::Uuid,
  pub tag_id: uuid::Uuid,
  pub content_id: uuid::Uuid,
  pub created_at: chrono::NaiveDateTime,
  pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "rich_contenttags"]
pub struct RichContentTag {
  pub idx: i32,
  pub contenttag_id: uuid::Uuid,
  pub tag_id: uuid::Uuid,
  pub content_id: uuid::Uuid,
  pub tag_title: String,
}