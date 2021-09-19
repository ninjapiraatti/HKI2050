use actix_files as fs;
//use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::Session;
use actix_web::http::{header, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
//use diesel::r2d2::{self, ConnectionManager};

mod handlers;

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
    println!("HTTP REQ:\n{:?}\n", req);
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

fn initialize_db(name: &str) {
	let connection = PgConnection::establish(&name).expect(&format!("Error connecting to {}", name));

	let result = diesel_migrations::run_pending_migrations(&connection);

	match result {
		Ok(_res) => {
			println!("Migrations done!");
		}
		Err(error) => {
			println!("Database migration error: \n {:#?}", error);
		}
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	initialize_db(&database_url);
	let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
	let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| "localhost:8086".to_string());
    println!("{:?}", domain);

	HttpServer::new(move || {
		App::new()
			.data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .service(
				web::scope("/api")
                .service(
                    web::resource("/test")
                        .route(web::get().to(handlers::test_handler::test)),
                )
            )
            .service(fs::Files::new("/public", "public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("HTTP REQ:\n{:?}\n", req);
				HttpResponse::Found().header(header::LOCATION, "index.html").finish()
			})))
	})
	.bind(server_url)?
	.run()
	.await
}
