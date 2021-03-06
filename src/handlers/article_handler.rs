use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool};
use crate::storage::*;
use crate::handlers::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ArticleData {
	pub user_id: uuid::Uuid,
	pub title: String,
	pub ingress: String,
	pub body: String,
	pub character_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct ArticleDeleteData {
	pub id: uuid::Uuid,
	pub character_id: uuid::Uuid,
	pub user_id: uuid::Uuid,
}

pub async fn add_article(
	uuid_path: web::Path<String>,
	article_data: web::Json<ArticleData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding an article: article_data = {:#?} logged_user = {:#?}",
		&article_data,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_path.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		articles_storage::create_article(
			id, 
			article_data.title.clone(), 
			article_data.ingress.clone(), 
			article_data.body.clone(), 
			article_data.character_id.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(article) => Ok(HttpResponse::Ok().json(&article)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
pub async fn get_articles(
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting articles");

	let res = web::block(move || articles_storage::query_articles(&pool)).await;
	match res {
		Ok(articles) => Ok(HttpResponse::Ok().json(&articles)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_uuid(
	id: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting article: article_data = {:#?} logged_user = {:#?}",
		&id,
		&logged_user
	);

	let user_id = logged_user.id;
	let article_id = uuid::Uuid::parse_str(&id.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || articles_storage::query_articles_by_article_uuid(article_id, &pool)).await;
	match res {
		Ok(article) => Ok(HttpResponse::Ok().json(&article)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_user_uuid(
	article_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting user articles: logged_user = {:#?}",
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || articles_storage::query_articles_by_user_uuid(user_id, &pool)).await;
	match res {
		Ok(article) => Ok(HttpResponse::Ok().json(&article)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_article(
	id: web::Path<String>,
	payload: web::Json<ArticleDeleteData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete article: article_data = {:#?} logged_user = {:#?}",
		&payload,
		&logged_user
	);

	let article_id = uuid::Uuid::parse_str(&id.into_inner())?;
	if logged_user.isadmin == false && logged_user.id != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || articles_storage::delete_article(article_id, &pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_article(
	id: web::Path<String>,
	payload: web::Json<ArticleData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating article: id = {:#?} payload = {:#?} logged_user = {:#?}",
		&id,
		&payload,
		&logged_user
	);

	let article_id = uuid::Uuid::parse_str(&id.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		articles_storage::update_article(
			article_id,
			payload.title.clone(),
			payload.ingress.clone(),
			payload.body.clone(),
			payload.character_id,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(userreservation) => Ok(HttpResponse::Ok().json(&userreservation)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

/*
pub fn delete_article(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::characters::dsl::*;

	let deleted = diesel::delete(characters.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
*/