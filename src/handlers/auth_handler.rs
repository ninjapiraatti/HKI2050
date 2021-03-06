use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::result::Error::NotFound;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::users::{LoggedUser, Pool, Session};
use crate::storage::*;
use crate::utils::verify;
use log::{debug, trace};

#[derive(Debug, Deserialize)]
pub struct AuthData {
	pub email: String,
	pub password: String,
}

pub async fn logout(
	id: Identity,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Logging out: id={:#?} logged_user={:#?}", &id.identity(), &logged_user);
	let res = web::block(move || sessions_storage::delete_session(logged_user.id, &pool)).await;
	id.forget();
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => match service_error {
				NotFound => Ok(HttpResponse::Ok().finish()),
				_ => Err(service_error.into()),
			},
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn login(
	auth_data: web::Json<AuthData>,
	id: Identity,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!("Logging in: email={:#?}", &auth_data.email);
	let res = web::block(move || query(auth_data.into_inner(), pool)).await;
	match res {
		Ok(user) => {
			id.remember(user.id.to_string());
			Ok(HttpResponse::Ok().finish())
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
	HttpResponse::Ok().json(logged_user.id)
}

fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<Session, ServiceError> {
	let res = users_storage::get_by_email(auth_data.email.clone(), &pool);

	match res {
		Ok(user) => {
			if let Ok(matching) = verify(&user.hash, &auth_data.password) {
				if matching {
					if let Ok(session) = sessions_storage::create_session(user.id.clone(), user.email.clone(), &pool) {
						return Ok(session);
					}
				}
			}
		}
		Err(NotFound) => {
			debug!("User not found");
		}
		Err(error) => {
			return Err(error.into());
		}
	}
	Err(ServiceError::Unauthorized)
}
