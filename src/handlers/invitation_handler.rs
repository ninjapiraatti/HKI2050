use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::result::Error::NotFound;
use log::{debug, trace};
use serde::Deserialize;

use crate::email_service::{send_invitation, send_reset_request};
use crate::errors::ServiceError;
use crate::models::invitations::{Invitation, Pool, ResetPasswordRequest};
use crate::storage::*;
use crate::utils::hash_password;

#[derive(Deserialize, Debug)]
pub struct InvitationData {
	pub email: String,
	pub password_plain: Option<String>,
	pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct ResetRequestData {
	pub email: String,
}

pub async fn post_invitation(
	invitation_data: web::Json<InvitationData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Posting invitation: invitation_data = {} {}",
		&invitation_data.email,
		&invitation_data.username,
	);
	let res = web::block(move || create_invitation(invitation_data.into_inner(), pool)).await;

	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn create_invitation(invdata: InvitationData, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let invitation = query_invitation(
		invdata.email,
		invdata.username,
		invdata.password_plain,
		pool,
	)?;
	send_invitation(&invitation)
}

pub async fn post_reset_request(
	reset_request_data: web::Json<ResetRequestData>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Posting reset_request: reset_request_data = {}",
		&reset_request_data.email
	);
	let res = web::block(move || create_reset_request(reset_request_data.into_inner(), pool)).await;

	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

fn create_reset_request(invdata: ResetRequestData, pool: web::Data<Pool>) -> Result<(), crate::errors::ServiceError> {
	let reset_request = query_reset_request(invdata.email, pool)?;
	send_reset_request(&reset_request)
}

fn query_invitation(
	eml: String,
	username: String,
	psw: Option<String>,
	pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
	let res_email = users_storage::get_by_email(eml.clone(), &pool);
	let res_username = users_storage::get_by_username(username.clone(), &pool);
	let password_hashed = psw.map(|s| hash_password(&s).unwrap());
	if res_email.is_ok() || res_username.is_ok() {
		debug!("User email or username already found. Cannot process invitation.");
		return Err(ServiceError::Unauthorized);
	} else {
		let mut reset_request_id: Option<uuid::Uuid> = None;
		let invitation = invitations_storage::create_invitation(
			eml,
			username,
			password_hashed,
			reset_request_id,
			&pool,
		)?;
		return Ok(invitation);
	}
}

fn query_reset_request(eml: String, pool: web::Data<Pool>) -> Result<ResetPasswordRequest, ServiceError> {
	let res = users_storage::get_by_email(eml.clone(), &pool);
	match res {
		Ok(_) => {
			let reset_request = reset_requests_storage::create_reset_request(eml, &pool)?;
			Ok(reset_request)
		}
		Err(NotFound) => {
			debug!("User ({}) not found. Cannot process reset request.", eml.clone());
			return Err(ServiceError::Unauthorized);
		}
		Err(error) => Err(error.into()),
	}
}
