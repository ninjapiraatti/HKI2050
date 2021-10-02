use super::super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
//use crate::schema::invitations::password_plain;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
	pub id: uuid::Uuid,
	pub email: String,
	pub username: String,
	pub password_plain: Option<String>,
	pub expires_at: chrono::NaiveDateTime,
	pub reset_request_id: Option<uuid::Uuid>,
	pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "reset_requests"]
pub struct ResetPasswordRequest {
	pub id: uuid::Uuid,
	pub email: String,
	pub expires_at: chrono::NaiveDateTime,
}

impl Invitation {
	pub fn from_details<S: Into<String>>(
		email: S,
		username: String,
		password_plain: Option<String>,
		reset_request_id: Option<uuid::Uuid>,
	) -> Self {
		let emailstr: String = email.into();
		Invitation {
			id: uuid::Uuid::new_v4(),
			email: String::from(&emailstr),
			username,
			password_plain,
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
			reset_request_id,
			updated_by: emailstr,
		}
	}
}

impl ResetPasswordRequest {
	pub fn from_details<S: Into<String>>(email: S) -> Self {
		let emailstr: String = email.into();
		ResetPasswordRequest {
			id: uuid::Uuid::new_v4(),
			email: String::from(&emailstr),
			expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
		}
	}
}
