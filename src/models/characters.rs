use super::super::schema::*;
//use crate::models;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "characters"]
pub struct Character {
	pub id: uuid::Uuid,
  pub user_id: uuid::Uuid,
	pub name: String,
  pub description: String,
  pub created_at: chrono::NaiveDateTime,
}