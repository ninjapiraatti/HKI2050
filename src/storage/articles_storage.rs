use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::PgConnection;

use crate::models::articles::Article;
use crate::models::users::Pool;
use diesel::result::Error;

pub fn create_article(
	q_user_id: uuid::Uuid,
	q_title: String,
	q_ingress: String,
	q_body: String,
	q_character_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Article, Error> {
	use crate::schema::articles::dsl::articles;
	let conn: &PgConnection = &pool.get().unwrap();

	let new_article = Article {
		id: uuid::Uuid::new_v4(),
		user_id: q_user_id,
		character_id: q_character_id,
		title: q_title,
		ingress: q_ingress,
		body: q_body,
		created_at: chrono::Local::now().naive_local(),
		updated_by: q_email,
	};

	let article = diesel::insert_into(articles)
		.values(&new_article)
		.get_result::<Article>(conn)?;

	Ok(article)
}

pub fn query_articles(
	pool: &web::Data<Pool>,
) -> Result<Vec<Article>, Error> {
	use crate::schema::articles::dsl::{articles};
	let conn: &PgConnection = &pool.get().unwrap();

	let articles_res = articles
		.load::<Article>(conn)?;

	Ok(articles_res)
}

pub fn query_articles_by_article_uuid(
	q_article_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<Article>, Error> {
	use crate::schema::articles::dsl::{id, articles};
	let conn: &PgConnection = &pool.get().unwrap();

	let articles_res = articles
		.filter(id.eq(&q_article_id))
		.load::<Article>(conn)?;

	Ok(articles_res)
}

pub fn query_articles_by_user_uuid(
	q_user_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<Article>, Error> {
	use crate::schema::articles::dsl::{user_id, articles};
	let conn: &PgConnection = &pool.get().unwrap();

	let articles_res = articles
		.filter(user_id.eq(&q_user_id))
		.load::<Article>(conn)?;

	Ok(articles_res)
}

/*
pub fn query_articles_by_tag_uuid(
	q_tag_id: uuid::Uuid,
	pool: &web::Data<Pool>,
) -> Result<Vec<Article>, Error> {
	use crate::schema::contenttags::dsl::{id, tag_id, contenttags};
	use crate::schema::articles::dsl::{id as article_id, articles};
	let conn: &PgConnection = &pool.get().unwrap();

	let articles_res = articles
		.filter(tag_id.eq(&q_tag_id))
		.load::<Article>(conn)?;

	Ok(articles_res)
}
*/

pub fn delete_article(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::articles::dsl::*;

	let deleted = diesel::delete(articles.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}

pub fn update_article(
	q_uuid_path: uuid::Uuid,
	q_title: String,
	q_ingress: String,
	q_body: String,
	q_character_id: uuid::Uuid,
	q_email: String,
	pool: &web::Data<Pool>,
) -> Result<Article, Error> {
	use crate::schema::articles::dsl::*;
	use crate::schema::articles::dsl::{id, updated_by};
	let conn: &PgConnection = &pool.get().unwrap();

	let user_article = diesel::update(articles)
		.filter(id.eq(q_uuid_path))
		.set((
			character_id.eq(q_character_id),
			title.eq(q_title),
			ingress.eq(q_ingress),
			body.eq(q_body),
			updated_by.eq(q_email),
		))
		.get_result::<Article>(conn)?;

	Ok(user_article)
}