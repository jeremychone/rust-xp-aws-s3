use aws_sdk_s3::primitives::ByteStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Setup & Fixtures
	let client = xp_aws_s3::new_client().await?;

	let bucket_name = "bucket-01";
	let file_name = "file-01.txt";
	let content = "content 01";

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
