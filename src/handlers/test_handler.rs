use actix_web::{error::Error, HttpResponse};

pub async fn test() -> Result<HttpResponse, Error> {
	println!("henlo");
	Ok(HttpResponse::Ok().body("ok".to_string()))
}
