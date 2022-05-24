use std::{error::Error, str};
use std::fs::File;
use std::io::{Read, Write};

mod minio;





use actix_web::{web, App, HttpServer};
//mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // Start http server
    HttpServer::new(move || {
        App::new()
            //.route("/minio", web::post().to(handlers::create_2ddoc))
			.route("/minio", web::post().to(handlers::minio_api))
			//.route("/web-user-interface", web::post().to())
    })
    //.bind("127.0.0.1:8080")?
	.bind("0.0.0.0:4100")?
    .run()
    .await
}


/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

	let bucket_name = "mybucket3";
	let key_file = "essai4";
	let file = "jocker.png";
	let content_file = convert_file(file);
	let data = minio::instantiate_bucket(bucket_name, content_file, key_file);	
	rebuild_file(data.await);

	Ok(())
	
}

pub fn convert_file(_file: &str) -> Vec<u8>  {
	// path
	let file_path = "/home/kali/Downloads/joker.png";	
	println!("path = {}", file_path);
	// open file
	let mut file = File::open(file_path).unwrap();
	let mut contents: Vec<u8> = vec![];
	//read all bytes and placing in buffer
	file.read_to_end(&mut contents).unwrap();
	println!("file convert to vec of byte");
	
	return contents;
}

pub fn rebuild_file(data: Vec<u8>) {
	// path
	let path = "/home/kali/Documents/SN_Docs-Management/newfile.jpeg";
	let _file_name = "newfile2.pdf";
	// create empty file
	let mut new_file = File::create(path).unwrap();
	// put and write data (vec of byte) in file
	new_file.write(&data).unwrap();
	println!("this file: {:?}", new_file);
}
*/