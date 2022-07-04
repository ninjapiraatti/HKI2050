use super::super::schema::*;
//use crate::models;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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