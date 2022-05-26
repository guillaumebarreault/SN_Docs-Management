use actix_web::{web, App, HttpServer};


mod handlers;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // Start http server
    HttpServer::new(move || {
        App::new()
			.route("/minio", web::post().to(handlers::minio_api))
			//.route("/web-user-interface", web::post().to())
    })
    .bind("127.0.0.1:8080")?
	//.bind("0.0.0.0:4100")?
    .run()
    .await
}