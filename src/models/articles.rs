use super::super::schema::*;
//use crate::models;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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