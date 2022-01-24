use crate::errors::ServiceError;
use crate::models::users::{User, LoggedUser, Pool};
use crate::storage::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NewUserData {
	pub email: String,
	pub password: String,
	pub username: String,
}

#[derive(Serialize, Debug)]
pub struct UserDTO {
	pub id: uuid::Uuid,
	pub username: String,
	pub isadmin: bool,
	pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryData {
	pub id: String,
	pub username: String,
	pub isadmin: bool,
	pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct ForgotPasswordData {
	pub email: String,
	pub password: String,
	pub id: uuid::Uuid,
}

pub async fn get_all(
	web::Query(q_query_data): web::Query<QueryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all users: logged_user = {:#?}", &logged_user);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || users_storage::query_all(&pool)).await;

	match res {
		Ok(users) => {
			if users.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&users));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_user(
	uuid_path: web::Path<String>,
	payload: web::Json<QueryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating a user: uuid_path = {:#?} logged_user = {:#?}",
		&uuid_path,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_path.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		users_storage::update(
			id,
			payload.username.clone(),
			payload.isadmin,
			payload.email.clone(),
			&pool,
		)
	})
	.await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_by_uuid(
	uuid_path: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Getting a user by uuid: uuid_path = {:#?} logged_user = {:#?}",
		&uuid_path,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_path.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || query_one(id.to_string(), pool)).await;

	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn query_one(uuid_path: String, pool: web::Data<Pool>) -> Result<UserDTO, ServiceError> {
	let uuid_query = uuid::Uuid::parse_str(&uuid_path)?;
	let user = users_storage::get(uuid_query, &pool)?;
	let data = UserDTO {
		id: user.id,
		username: user.username,
		isadmin: user.isadmin,
		email: user.email,
	};
	if data.id.is_nil() == false {
		return Ok(data.into());
	}
	Err(ServiceError::Empty)
}

pub async fn delete_user(
	uuid_path: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a user: uuid_path = {:#?} logged_user = {:#?}",
		&uuid_path,
		&logged_user
	);

	let id = uuid::Uuid::parse_str(&uuid_path.into_inner())?;

	if logged_user.isadmin == false && logged_user.id != id {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || users_storage::delete_user(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_password(
	payload: web::Json<ForgotPasswordData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Resetting password for: email = {:#?}", &payload.email,);

	let id = reset_requests_storage::get_by_reset_request(payload.id.clone(), &pool);
	match id {
		Ok(_) => println!("Reset request found"),
		Err(_) => return Err(ServiceError::InternalServerError),
	}

	let res =
		web::block(move || users_storage::set_password(payload.email.clone(), payload.password.clone(), &pool))
			.await;
	match res {
		Ok(user) => Ok(HttpResponse::Ok().json(&user)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}