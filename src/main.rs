#[macro_use]
extern crate diesel;

use actix_files as fs;
use aninmals;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::Session;
use actix_web::http::{header, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::{error, info, trace};
//use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod handlers;
mod models;
mod schema;
mod storage;
mod utils;
mod email_service;

#[get("/")]
async fn home(session: Session) -> Result<HttpResponse> {
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		trace!("SESSION value: {}", count);
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

#[get("/app/*")]
async fn allviews(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	trace!("allviews: {:?}", req);
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		trace!("SESSION value: {}", count);
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

fn initialize_db(name: &str) {
	info!("Running database migrations...");
	let connection = PgConnection::establish(&name).expect(&format!("Error connecting to {}", name));

	let result = diesel_migrations::run_pending_migrations(&connection);

	match result {
		Ok(_res) => {
			println!("Migrations done!");
		}
		Err(error) => {
			error!("Database migration error: \n {:#?}", error);
		}
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	aninmals::get_random_aninmal();
	dotenv::dotenv().ok();
	let rust_log = std::env::var("RUST_LOG").unwrap_or("info, simple-auth-server=debug".to_string());
	std::env::set_var("RUST_LOG", rust_log);
	env_logger::init();
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	initialize_db(&database_url);
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool: models::users::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
	let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "hki2050.com".to_string());
	let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| "localhost:8086".to_string());
	println!("{:?}", domain);

	HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			//.data(web::JsonConfig::default().limit(4096))
			.wrap(middleware::Logger::default())
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
					.name("auth")
					.path("/")
					.domain(domain.as_str())
					.max_age(86400)
					.secure(false),
			))
			.service(
				web::scope("/api")
					.service(
						web::resource("/invitations")
							.route(web::post().to(handlers::invitation_handler::post_invitation)),
					)
					.service(
						web::resource("/test")
							.route(web::post().to(handlers::test_handler::test)),
					)
					.service(
						web::resource("/register/{invitation_id}")
							.route(web::post().to(handlers::register_handler::register_user)),
					)
					.service(
						web::resource("/resetpassword")
							.route(web::post().to(handlers::invitation_handler::post_reset_request)),
					)
					.service(
						web::resource("/updatepassword").route(web::put().to(handlers::users_handler::update_password)),
					)
					.service(
						web::resource("/users/{user_id}")
							.route(web::get().to(handlers::users_handler::get_by_uuid))
							.route(web::put().to(handlers::users_handler::update_user))
							.route(web::delete().to(handlers::users_handler::delete_user)),
					)

					// Characters

					.service(
						web::resource("/users/{user_id}/characters")
							.route(web::get().to(handlers::character_handler::get_by_user_uuid))
							.route(web::post().to(handlers::character_handler::add_character))
					)
					.service(
						web::resource("/users/characters/{character_id}")
							.route(web::put().to(handlers::character_handler::update_character))
							.route(web::delete().to(handlers::character_handler::delete_character)),
					)
					.service(
						web::resource("/characters/{character_id}")
							.route(web::get().to(handlers::character_handler::get_by_character_uuid))
					)

					// Articles

					.service(
						web::resource("/users/{user_id}/articles")
							.route(web::get().to(handlers::article_handler::get_by_user_uuid))
							.route(web::post().to(handlers::article_handler::add_article)),
					)
					.service(
						web::resource("/users/articles/{article_id}")
							.route(web::put().to(handlers::article_handler::update_article))
							.route(web::delete().to(handlers::article_handler::delete_article)),
					)
					.service(
						web::resource("/articles/{article_id}")
							.route(web::get().to(handlers::article_handler::get_by_uuid))
					)
					.service(
						web::resource("/articles")
							.route(web::get().to(handlers::article_handler::get_articles)),
					)

					// Tags

					.service(
						web::resource("/tags")
							.route(web::get().to(handlers::tag_handler::get_tags))
							.route(web::post().to(handlers::tag_handler::add_tag))
					)
					.service(
						web::resource("/tags/{tag_id}")
							//.route(web::get().to(handlers::tag_handler::get_articles_by_tag))
							.route(web::put().to(handlers::tag_handler::update_tag))
							.route(web::delete().to(handlers::tag_handler::delete_tag)),
					)

					// Article specific tags

					.service(
						web::resource("/content-tags/{content_id}")
							.route(web::get().to(handlers::tag_handler::get_content_tags))
							.route(web::post().to(handlers::tag_handler::add_content_tag))
					)
					.service(
						web::resource("/content-tags/{tag_id}")
							.route(web::delete().to(handlers::tag_handler::delete_content_tag)),
					)

					// Auth

					.service(
						web::resource("/auth")
							.route(web::post().to(handlers::auth_handler::login))
							.route(web::delete().to(handlers::auth_handler::logout))
							.route(web::get().to(handlers::auth_handler::get_me)),
					),
			)
			.service(fs::Files::new("/public", "public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
				trace!("HTTP REQ:\n{:?}\n", req);
				HttpResponse::Found().header(header::LOCATION, "index.html").finish()
			})))
	})
	.bind(server_url)?
	.run()
	.await
}
