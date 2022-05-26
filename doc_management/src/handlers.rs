use actix_web::Responder;
use actix_web::{web};
use serde::{Deserialize, Serialize};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::BucketConfiguration;




#[derive(Debug, Serialize, Deserialize)]
pub struct InputDoc {
    pub uid: String,
	//pub token: String,
    pub doc_name: String,
	pub content_doc: Vec<u8>,
}


// 
pub async fn minio_api(item: web::Json<InputDoc>) -> impl Responder {
	let uid = &item.uid;
	let doc_name = &item.doc_name;
	let content_doc = &item.content_doc;
	
	instantiate_bucket(uid, doc_name, content_doc.to_vec());
}


// Instantiate the bucket client
pub async fn instantiate_bucket(bucket_name: &str, key: &str, content_object: Vec<u8>) -> Vec<u8> {
	// Init bucket and minio route
	let bucket = Bucket::new_with_path_style(
		bucket_name,
		Region::Custom {
			region: "".to_owned(),
			endpoint: "http://127.0.0.1:9000".to_owned(),
		},
		Credentials {
			access_key: Some("minio".to_owned()), // get of /etc/default/minio
			secret_key: Some("SN-minio-serv".to_owned()), // get of /etc/default/minio
			security_token: None,
			session_token: None,
		},
	).unwrap();
	// Create bucket if does not exist
	let (_, code) = bucket.head_object("/").await.unwrap();
	if code == 404 {
		let create_result = Bucket::create_with_path_style(
			bucket.name.as_str(),
			bucket.region.clone(),
			bucket.credentials.clone(),
			BucketConfiguration::default(),
		)
		.await.unwrap();
		println!("\nBucket created:\n{} - {} - {}\n",bucket.name, create_result.response_code, create_result.response_text);
	}
	println!("\n{} already created", bucket.name);

	
	put_content_bucket(&bucket, key, content_object);	
	//let list = list_bucket_content(&bucket);
	let data = get_content_bucket(&bucket, key);
		
	return data.await;
}


// List of contents in bucket
pub async fn list_bucket_content(bucket: &Bucket) {
	println!("\nList bucket content:");
	let results = bucket.list("/".to_owned(), Some("/".to_owned())).await.unwrap();
	for result in results {
		for item in result.contents {
			println!("key: {}", item.key);
		}
	}
}


// Put content in bucket
pub async fn put_content_bucket(bucket: &Bucket, key: &str, content_object: Vec<u8>) {
	println!("\nPut content:");
	bucket
		.put_object_with_content_type(key, content_object.as_ref(), "text/plain")
		.await.unwrap();
	println!("file uploaded");
}


// Get object content from bucket
pub async fn get_content_bucket(bucket: &Bucket, key: &str) -> Vec<u8> {
	println!("\nGet content:");
	let (data, _) = bucket.get_object(key).await.unwrap();

	return data;
}