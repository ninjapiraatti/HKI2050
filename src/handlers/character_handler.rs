use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool};
//use crate::models::characters::{Character};
use crate::storage::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CharacterData {
	pub name: String,
	pub description: String,
}

pub async fn add_character(
	character_data: web::Json<CharacterData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a character: character_data = {:#?} logged_user = {:#?}",
		&character_data,
		&logged_user
	);

	let user_id = logged_user.id;

	let res = web::block(move || {
		characters_storage::create_character(
      user_id, 
      character_data.name.clone(), 
      character_data.description.clone(), 
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

pub async fn get_by_character_uuid(
	character_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Adding a favorite project: character_data = {:#?} logged_user = {:#?}",
		&character_data,
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || characters_storage::query_characters_by_character_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_user_uuid(
	character_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting user characters: logged_user = {:#?}",
		&logged_user
	);

	let user_id = logged_user.id;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || characters_storage::query_characters_by_user_uuid(user_id, &pool)).await;
	match res {
		Ok(project) => Ok(HttpResponse::Ok().json(&project)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_character(
	character_data: web::Path<(String, String)>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Delete a favorite project: character_data = {:#?} logged_user = {:#?}",
		&character_data,
		&logged_user
	);

	let input = character_data.into_inner();
	let user_id = uuid::Uuid::parse_str(&input.0)?;

	if logged_user.isadmin == false && logged_user.id != user_id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || characters_storage::delete_character(user_id, &pool)).await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}