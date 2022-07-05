use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::PgConnection;

use crate::models::tags::{Tag, ContentTag, RichContentTag};
use crate::models::users::Pool;
use diesel::result::Error;

pub fn query_tags(
	pool: &web::Data<Pool>,
) -> Result<Vec<Tag>, Error> {
	use crate::schema::tags::dsl::{tags};
	let conn: &PgConnection = &pool.get().unwrap();

	let tags_res = tags
		.load::<Tag>(conn)?;

	Ok(tags_res)
}

pub fn query_content_tags(
	q_content_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<RichContentTag>, Error> {
	use crate::schema::rich_contenttags::dsl::{content_id, rich_contenttags};
	let conn: &PgConnection = &pool.get().unwrap();

	let tags_res = rich_contenttags
		.filter(content_id.eq(&q_content_id))
		.load::<RichContentTag>(conn)?;

	Ok(tags_res)
}

pub fn create_tag(
	q_title: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Tag, Error> {
	use crate::schema::tags::dsl::tags;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_tag = Tag {
		id: uuid::Uuid::new_v4(),
		title: q_title,
		updated_by: q_email,
		created_at: chrono::Local::now().naive_local(),
	};

	let tag = diesel::insert_into(tags)
		.values(&new_tag)
		.get_result::<Tag>(conn)?;

	Ok(tag)
}

pub fn create_content_tag(
	q_tag_id: uuid::Uuid,
	q_content_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<ContentTag, Error> {
	use crate::schema::contenttags::dsl::contenttags;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_tag = ContentTag {
		id: uuid::Uuid::new_v4(),
		tag_id: q_tag_id,
		content_id: q_content_id,
		updated_by: q_email,
		created_at: chrono::Local::now().naive_local(),
	};

	let tag = diesel::insert_into(contenttags)
		.values(&new_tag)
		.get_result::<ContentTag>(conn)?;

	Ok(tag)
}

pub fn delete_tag(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::articles::dsl::*;

	let deleted = diesel::delete(articles.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}

pub fn update_tag(
	q_uuid: uuid::Uuid,
	q_title: String,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Tag, Error> {
	use crate::schema::tags::dsl::*;
	use crate::schema::tags::dsl::{id, updated_by};
	let conn: &PgConnection = &pool.get().unwrap();

	let tag = diesel::update(tags)
		.filter(id.eq(q_uuid))
		.set((
			title.eq(q_title),
			updated_by.eq(q_email),
		))
		.get_result::<Tag>(conn)?;

	Ok(tag)
}