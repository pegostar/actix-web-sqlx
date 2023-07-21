mod model;
mod schema;
mod middlewares;
mod handler;


use std::net::Ipv4Addr;
use actix_cors::Cors;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use actix_web::{http::header, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::{PgPool, PgPoolOptions};
use slog::info;
use crate::middlewares::configure_log;

//https://github.com/juhaku/utoipa/blob/master/examples/todo-actix/src/main.rs
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sample Actix Web and SQLX",
        description = "Full sample using actix web and sqlx",
        contact(name= "Davide Pegoraro",email= "davide.pegoraro@outlook.com"),
        license(name="Apache 2.0"),
        version = "1.0.0"
    ),
    paths(
        handler::people_list_handler,
        handler::get_people_handler,
        handler::create_people_handler,
        handler::edit_people_handler,
        handler::delete_people_handler,
    ),
    components(
        schemas(model::PeopleModel, model::PeopleModelView)
    ),
)]
struct ApiDoc;

pub struct AppState {
    db: PgPool,
    log: slog::Logger
}

async fn get_health_status(data: web::Data<AppState>) -> HttpResponse{
    let is_database_connected = sqlx::query("SELECT 1")
        .fetch_one(&data.db)
        .await
        .is_ok();

    if is_database_connected {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::json!({ "database_connected": is_database_connected }).to_string())
    } else {
        HttpResponse::ServiceUnavailable()
            .content_type("application/json")
            .body(serde_json::json!({ "database_connected": is_database_connected }).to_string())
    }
}


//https://dev.to/chaudharypraveen98/adding-slog-logger-to-actix-web-2332

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    //env_logger::init();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let port= std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    let log = configure_log();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    info!(log,
        "🚀 Server started successfully"
    );

    HttpServer::new(move || {

        let cors = Cors::default()
            /*.allowed_origin("http://localhost:8000")*/
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();


        //let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone(), log: log.clone() }))
            .configure(handler::config)
            /*http://localhost:8000/swagger-ui/index.html*/
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/health", web::get().to(get_health_status))
            .wrap(cors)
            .wrap( middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            /*.wrap(Logger::default())*/
    })
        .bind((Ipv4Addr::UNSPECIFIED, port.parse().unwrap()))?
        .run()
        .await
}