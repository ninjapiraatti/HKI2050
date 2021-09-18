#[macro_use]

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::Session;
use actix_web::http::{header, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};

#[get("/")]
async fn home(session: Session) -> Result<HttpResponse> {
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

#[get("/app/*")]
async fn allviews(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
    env_logger::init();
	let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
	let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| "localhost:8086".to_string());

	HttpServer::new(move || {
		App::new()
			.data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            /*
			.service(
				web::scope("/api")
					.service(
						web::resource("/register/{invitation_id}")
							.route(web::post().to(handlers::register_handler::register_user)),
					)
					.service(web::resource("/upload").route(web::post().to(handlers::upload_handler::save_file)))
					.service(
						web::resource("/auth")
							.route(web::post().to(handlers::auth_handler::login))
							.route(web::delete().to(handlers::auth_handler::logout))
							.route(web::get().to(handlers::auth_handler::get_me)),
					),
			)*/
			//.service(fs::Files::new("/public", "public").show_files_listing())
            .service(fs::Files::new("/public", "./public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
				HttpResponse::Found().header(header::LOCATION, "index.html").finish()
			})))
	})
	.bind(server_url)?
	.run()
	.await
}
