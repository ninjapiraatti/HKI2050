use super::super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "characters"]
pub struct Character {
	pub id: uuid::Uuid,
  pub user_id: uuid::Uuid,
	pub name: String,
  pub description: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_by: String,
}