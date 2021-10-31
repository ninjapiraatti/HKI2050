use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::PgConnection;

use crate::models::characters::Character;
use crate::models::users::Pool;
use diesel::result::Error;

pub fn create_character(
	q_user_id: uuid::Uuid,
	q_name: String,
	q_description: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Character, Error> {
	use crate::schema::characters::dsl::characters;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_character = Character {
		id: uuid::Uuid::new_v4(),
		user_id: q_user_id,
		name: q_name,
		description: q_description,
		created_at: chrono::Local::now().naive_local(),
		updated_by: q_email,
	};

	let character = diesel::insert_into(characters)
		.values(&new_character)
		.get_result::<Character>(conn)?;

	Ok(character)
}

pub fn query_characters_by_character_uuid(
	q_user_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<Character>, Error> {
	use crate::schema::characters::dsl::{user_id, characters};
	let conn: &PgConnection = &pool.get().unwrap();

	let characters_res = characters
		.filter(user_id.eq(&q_user_id))
		.load::<Character>(conn)?;

	Ok(characters_res)
}

pub fn query_characters_by_user_uuid(
	q_user_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<Character>, Error> {
	use crate::schema::characters::dsl::{user_id, characters};
	let conn: &PgConnection = &pool.get().unwrap();

	let characters_res = characters
		.filter(user_id.eq(&q_user_id))
		.load::<Character>(conn)?;

	Ok(characters_res)
}

pub fn delete_character(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::characters::dsl::*;

	let deleted = diesel::delete(characters.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}

pub fn update_character(
	q_uuid_data: uuid::Uuid,
	q_name: String,
	q_description: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Character, Error> {
	use crate::schema::characters::dsl::*;
	use crate::schema::characters::dsl::{id, updated_by};
	let conn: &PgConnection = &pool.get().unwrap();

	let user_character = diesel::update(characters)
		.filter(id.eq(q_uuid_data))
		.set((
			name.eq(q_name),
			description.eq(q_description),
			updated_by.eq(q_email),
		))
		.get_result::<Character>(conn)?;

	Ok(user_character)
}