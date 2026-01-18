use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Setup & Fixtures
	let bucket_name = "bucket-01";
	let file_name = "file-01.txt";
	let content = "content 01";

	// Using hardcoded credentials as requested (matching c01-list.rs setup).
	let creds = Credentials::new("rustfsadmin", "rustfsadmin", None, None, "hardcoded");
	let region = Region::new("us-east-1");

	let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
		.region(region)
		.endpoint_url("http://127.0.0.1:9000")
		.credentials_provider(creds)
		.load()
		.await;

	let client = Client::new(&config);

	// -- Exec
	// 1. Ensure bucket exists
	let resp = client.list_buckets().send().await?;
	let bucket_exists = resp.buckets().iter().any(|b| b.name() == Some(bucket_name));

	if !bucket_exists {
		println!("Bucket '{bucket_name}' not found. Creating...");
		client.create_bucket().bucket(bucket_name).send().await?;
	} else {
		println!("Bucket '{bucket_name}' already exists.");
	}

	// 2. Create or overwrite the file
	println!("Uploading '{file_name}' to '{bucket_name}'...");
	client
		.put_object()
		.bucket(bucket_name)
		.key(file_name)
		.body(ByteStream::from(content.as_bytes().to_vec()))
		.send()
		.await?;

	// -- Output
	println!("Successfully ensured bucket '{bucket_name}' and created/updated '{file_name}'.");

	Ok(())
}
