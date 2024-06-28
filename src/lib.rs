use aws_config::SdkConfig;
use aws_sdk_sts::{error::SdkError, types::Credentials, Client};

mod error;
use error::GenerateTemporaryCredentialsError;

/// The duration in seconds for which temporary AWS credentials should be valid.
pub const VALIDITY: i32 = 15 * 60;

/// Generates temporary AWS credentials using the provided AWS SDK client.
///
/// This function retrieves temporary AWS credentials from the AWS STS service using the provided AWS SDK client.
/// The credentials are valid for the duration specified by the [`VALIDITY`] constant.
pub async fn generate_temporary_credentials(
    client: &Client,
) -> Result<Credentials, GenerateTemporaryCredentialsError> {
    client
        .get_session_token()
        .duration_seconds(VALIDITY)
        .send()
        .await
        .map_err(SdkError::into_service_error)
        .map_err(GenerateTemporaryCredentialsError::from)?
        .credentials
        .ok_or(GenerateTemporaryCredentialsError::CredentialsNotReturned)
}

/// Generates temporary AWS credentials using the provided AWS SDK configuration.
///
/// This function retrieves temporary AWS credentials from the AWS STS service using the provided AWS SDK configuration.
/// The credentials are valid for the duration specified by the [`VALIDITY`] constant.
pub async fn generate_temporary_credentials_with_config(
    config: &SdkConfig,
) -> Result<Credentials, GenerateTemporaryCredentialsError> {
    let client = Client::new(config);
    generate_temporary_credentials(&client).await
}

/// Generates temporary AWS credentials using the AWS SDK configuration loaded from the environment.
///
/// This function retrieves temporary AWS credentials from the AWS STS service using the AWS SDK configuration
/// loaded from the environment variables. The credentials are valid for the duration specified by the [`VALIDITY`]
/// constant.
pub async fn generate_temporary_credentials_with_config_from_env(
) -> Result<Credentials, GenerateTemporaryCredentialsError> {
    let config = aws_config::load_from_env().await;
    generate_temporary_credentials_with_config(&config).await
}
