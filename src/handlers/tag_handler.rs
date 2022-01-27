use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool};
use crate::storage::*;
use crate::handlers::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TagData {
	pub user_id: uuid::Uuid,
	pub title: String,
}

pub async fn get_tags(
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting tags");

	let res = web::block(move || tags_storage::query_tags(&pool)).await;
	match res {
		Ok(tag) => Ok(HttpResponse::Ok().json(&tag)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn add_tag(
	tag_data: web::Json<TagData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding an tag: tag_data = {:#?} logged_user = {:#?}",
		&tag_data,
		&logged_user
	);

	if logged_user.isadmin == false && logged_user.id != tag_data.user_id {
		return Err(ServiceError::AdminRequired);
	};

	let res = web::block(move || {
		tags_storage::create_tag(
			tag_data.title.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(tag) => Ok(HttpResponse::Ok().json(&tag)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

/*
pub async fn get_articles_by_tag(
	tag_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a favorite project: tag_data = {:#?} logged_user = {:#?}",
		&tag_data,
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || articles_storage::query_articles_by_tag_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
*/

pub async fn delete_tag(
	id: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete a tag: logged_user = {:#?}",
		&logged_user
	);

	let tag_id = uuid::Uuid::parse_str(&id.into_inner())?;
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || tags_storage::delete_tag(tag_id, &pool)).await;
	match res {
		Ok(tag) => Ok(HttpResponse::Ok().json(&tag)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_tag(
	id: web::Path<String>,
	payload: web::Json<TagData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating tag: id = {:#?} payload = {:#?} logged_user = {:#?}",
		&id,
		&payload,
		&logged_user
	);

	let tag_id = uuid::Uuid::parse_str(&id.into_inner())?;

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		tags_storage::update_tag(
			tag_id,
			payload.title.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(tag) => Ok(HttpResponse::Ok().json(&tag)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}