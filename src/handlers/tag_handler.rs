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

pub async fn add_tag(
	user_id: web::Path<String>,
	tag_data: web::Json<TagData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding an tag: tag_data = {:#?} logged_user = {:#?}",
		&tag_data,
		&logged_user
	);

	let user_id = logged_user.id;
	let id = uuid::Uuid::parse_str(&character_id.into_inner())?;

	let res = web::block(move || {
		tags_storage::create_tag(
			user_id, 
			tag_data.title.clone(), 
			tag_data.ingress.clone(), 
			tag_data.body.clone(), 
			id,
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

pub async fn get_by_uuid(
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

	let res = web::block(move || tags_storage::query_tags_by_tag_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_character_uuid(
	tag_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting user tags: logged_user = {:#?}",
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || tags_storage::query_tags_by_user_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_tag(
	id: web::Path<String>,
	payload: web::Json<tagDeleteData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete a favorite project: tag_data = {:#?} logged_user = {:#?}",
		&payload,
		&logged_user
	);

	let tag_id = uuid::Uuid::parse_str(&id.into_inner())?;
	if logged_user.isadmin == false && logged_user.id != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || tags_storage::delete_tag(tag_id, &pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_tag(
	id: web::Path<String>,
	payload: web::Json<tagData>,
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

	if logged_user.isadmin == false && logged_user.id != payload.user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		tags_storage::update_tag(
			tag_id,
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
pub fn delete_tag(q_id: uuid::Uuid, pool: &web::Data<Pool>) -> Result<(), Error> {
	let conn: &PgConnection = &pool.get().unwrap();
	use crate::schema::characters::dsl::*;

	let deleted = diesel::delete(characters.filter(id.eq(q_id))).execute(conn)?;

	if deleted > 0 {
		return Ok(());
	}
	Err(NotFound)
}
*/