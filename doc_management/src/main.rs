use actix_web::{web, App, HttpServer};

mod handlers;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/miniopush", web::post().to(handlers::minio_api))
			//.route("/web-user-interface", web::post().to())
    })
	.bind("0.0.0.0:5100")?
    .run()
    .await
}