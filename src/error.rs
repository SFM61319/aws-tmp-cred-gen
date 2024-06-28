use aws_sdk_sts::operation::get_session_token::GetSessionTokenError;

/// Represents errors that can occur when generating temporary AWS credentials.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenerateTemporaryCredentialsError {
    /// An unknown error occurred.
    #[default]
    Unknown,

    /// The AWS region is disabled.
    RegionDisabled,

    /// The AWS credentials were not returned.
    CredentialsNotReturned,
}

impl From<GetSessionTokenError> for GenerateTemporaryCredentialsError {
    fn from(value: GetSessionTokenError) -> Self {
        value
            .is_region_disabled_exception()
            .then_some(GenerateTemporaryCredentialsError::RegionDisabled)
            .unwrap_or_default()
    }
}
