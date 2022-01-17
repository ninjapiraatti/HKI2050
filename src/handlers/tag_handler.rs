use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool};
use crate::storage::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ArticleData {
	pub title: String,
	pub ingress: String,
	pub body: String,
	pub tags: Vec<String>,
	pub user_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct ArticleDeleteData {
	pub id: uuid::Uuid,
	pub user_id: uuid::Uuid,
}

pub async fn add_article(
	article_data: web::Json<ArticleData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding an article: article_data = {:#?} logged_user = {:#?}",
		&article_data,
		&logged_user
	);

	let user_id = logged_user.id;

	let res = web::block(move || {
		articles_storage::create_article(
      user_id, 
      article_data.name.clone(), 
      article_data.description.clone(), 
			logged_user.email,
      &pool,
    )
	})
	.await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_article_uuid(
	article_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a favorite project: article_data = {:#?} logged_user = {:#?}",
		&article_data,
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || articles_storage::query_articles_by_article_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_character_uuid(
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
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_article(
	id: web::Path<String>,
	payload: web::Json<articleDeleteData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete a favorite project: article_data = {:#?} logged_user = {:#?}",
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
	payload: web::Json<articleData>,
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
			payload.name.clone(),
			payload.description.clone(),
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