#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Setup & Fixtures
	let client = xp_aws_s3::new_client().await?;

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
