use aws_config::SdkConfig;
use aws_sdk_sts::{types::Credentials, Client};

/// The duration in seconds for which temporary AWS credentials should be valid.
pub const VALIDITY: i32 = 15 * 60;

/// Generates temporary AWS credentials using the provided AWS SDK client.
///
/// This function uses the AWS SDK to generate temporary AWS credentials with a
/// validity period of [`VALIDITY`] seconds.
///
/// # Arguments
///
/// * `client` - The AWS SDK client to use for generating the temporary credentials.
///
/// # Returns
///
/// A [`Result`] containing the generated [`Credentials`] or an error message.
pub async fn generate_temporary_credentials(client: &Client) -> Result<Credentials, &'static str> {
    client
        .get_session_token()
        .duration_seconds(VALIDITY)
        .send()
        .await
        .map_err(|err| {
            if err.into_service_error().is_region_disabled_exception() {
                "region disabled"
            } else {
                "unknown"
            }
        })?
        .credentials
        .ok_or("credentials not returned")
}

/// Generates temporary AWS credentials using the provided AWS SDK configuration.
///
/// This function uses the AWS SDK to generate temporary AWS credentials with a
/// validity period of [`VALIDITY`] seconds.
///
/// # Arguments
///
/// * `config` - The AWS SDK configuration to use for generating the temporary credentials.
///
/// # Returns
///
/// A [`Result`] containing the generated [`Credentials`] or an error message.
pub async fn generate_temporary_credentials_with_config(
    config: &SdkConfig,
) -> Result<Credentials, &'static str> {
    let client = Client::new(config);
    generate_temporary_credentials(&client).await
}

/// Generates temporary AWS credentials using the AWS SDK configuration loaded from the environment.
///
/// This function loads the AWS SDK configuration from the environment and uses it to generate temporary AWS
/// credentials with a validity period of [`VALIDITY`] seconds.
///
/// # Returns
///
/// A [`Result`] containing the generated [`Credentials`] or an error message.
pub async fn generate_temporary_credentials_with_config_from_env(
) -> Result<Credentials, &'static str> {
    let config = aws_config::load_from_env().await;
    generate_temporary_credentials_with_config(&config).await
}
