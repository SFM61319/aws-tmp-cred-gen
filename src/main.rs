use aws_tmp_cred_gen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials =
        aws_tmp_cred_gen::generate_temporary_credentials_with_config_from_env().await?;

    println!("Access Key ID: {}", credentials.access_key_id);
    println!("Secret Access Key: {}", credentials.secret_access_key);
    println!("Session Token: {}", credentials.session_token);
    println!("Expiration: {}", credentials.expiration);

    Ok(())
}
