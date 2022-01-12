use crate::models::users::{Pool, User};
use crate::utils::hash_password;
use actix_web::web;
use diesel::result::Error;
use diesel::{prelude::*, PgConnection};
use log::{info};
use Error::NotFound;

pub fn query_all(pool: &web::Data<Pool>) -> Result<Vec<User>, Error> {
	use crate::schema::users::dsl::{username, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let items = users.order(username.asc()).load::<User>(conn)?;

	Ok(items)
}

pub fn get_by_email(q_email: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{email, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(email.eq(&q_email)).get_result::<User>(conn)?;
	info!("USER from email: {}", user.email);
	Ok(user)
}

pub fn get_by_username(q_username: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{username, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(username.eq(&q_username)).get_result::<User>(conn)?;
	info!("USER by username: {}", user.username);
	Ok(user)
}

pub fn get(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{id, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = users.filter(id.eq(&q_id)).get_result::<User>(conn)?;

	Ok(user)
}

pub fn create(q_email: String, q_username: String, q_password: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::users;

	let conn: &PgConnection = &pool.get().unwrap();

	let new_user = User::from_details(q_email, q_password, q_username);

	let user: User = diesel::insert_into(users).values(&new_user).get_result(conn)?;

	Ok(user)
}

pub fn update(
	uuid_data: uuid::Uuid,
	q_username: String,
	q_user_is_admin: bool,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<User, Error> {
	use crate::schema::users::dsl::{id, isadmin, username, users};
	let conn: &PgConnection = &pool.get().unwrap();

	let user = diesel::update(users)
		.filter(id.eq(uuid_data))
		.set((username.eq(q_username), isadmin.eq(q_user_is_admin)))
		.get_result::<User>(conn)?;

	Ok(user)
}

pub fn set_password(q_email: String, q_password: String, pool: &web::Data<Pool>) -> Result<User, Error> {
	use crate::schema::users::dsl::{email, hash, users};
	let conn: &PgConnection = &pool.get().unwrap();
	let password_hashed = hash_password(&q_password).unwrap();

	let user = diesel::update(users)
		.filter(email.eq(q_email.clone()))
		.set(hash.eq(password_hashed))
		.get_result::<User>(conn)?;

	Ok(user)
}

pub fn delete_user(uuid_data: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::users::dsl::id;
	use crate::schema::users::dsl::*;

	let deleted = diesel::delete(users.filter(id.eq(uuid_data))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
