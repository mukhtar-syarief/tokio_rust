use actix_web::{HttpServer, App, get, middleware::Logger};
use env_logger::Env;
use actix_cors::Cors;

#[get("/")]
pub async fn test_health() -> String{
    format!("Ararara")
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    println!("Server running on http://127.0.0.1:3030\n");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials()
        )
        .wrap(Logger::default())
            .service(test_health)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
