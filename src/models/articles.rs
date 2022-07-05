use super::super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "articles"]
pub struct Article {
	pub id: uuid::Uuid,
  pub character_id: uuid::Uuid,
  pub user_id: uuid::Uuid,
	pub title: String,
  pub ingress: String,
  pub body: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_by: String,
}