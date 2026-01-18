use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Setup & Fixtures
	// Using hardcoded credentials as requested.
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
	let resp = client.list_buckets().send().await?;

	// -- Output
	let buckets = resp.buckets();

	println!("Found {} bucket(s):", buckets.len());
	for bucket in buckets {
		if let Some(name) = bucket.name() {
			println!(" - {name}");
		}
	}

	Ok(())
}
