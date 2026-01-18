// region:    --- Modules

mod error;

use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
pub use error::{Error, Result};

// endregion: --- Modules

pub async fn new_client() -> Result<Client> {
	let creds = Credentials::new("rustfsadmin", "rustfsadmin", None, None, "hardcoded");
	let region = Region::new("us-east-1");

	let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
		.region(region)
		.endpoint_url("http://127.0.0.1:9000")
		.credentials_provider(creds)
		.load()
		.await;

	Ok(Client::new(&config))
}
