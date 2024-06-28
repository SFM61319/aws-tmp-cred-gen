use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use tokio::net::TcpListener;

use aws_tmp_cred_gen::generate_temporary_credentials_with_config_from_env;

/// The address on which the server will listen for incoming connections.
pub const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

/// Starts the server and listens for incoming connections on the specified address.
///
/// This function creates a new Axum router with a route for the root path (`/`) that
/// handles requests using the [`root_handler`] function. It then binds a TCP listener
/// to the [`ADDRESS`] constant and starts serving the application using the Axum server.
/// The server will continue running until it is stopped or encounters an error.
pub async fn serve() {
    let app = Router::new().route("/", get(root_handler));
    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    println!("Listening on http://{}/", ADDRESS);
    axum::serve(listener, app).await.unwrap();
}

/// Generates and returns temporary AWS credentials.
///
/// This function uses the [`aws_tmp_cred_gen`] crate to generate temporary AWS credentials
/// based on the environment configuration. It returns a JSON object containing the
/// access key ID, secret access key, session token, and expiration date of the
/// generated credentials.
///
/// If an error occurs during the credential generation process, this function will
/// return a [`StatusCode::INTERNAL_SERVER_ERROR`] error.
async fn root_handler() -> Result<Json<Value>, StatusCode> {
    generate_temporary_credentials_with_config_from_env()
        .await
        .map(|credentials| {
            Json(json!({
                "accessKeyId": credentials.access_key_id,
                "secretAccessKey": credentials.secret_access_key,
                "sessionToken": credentials.session_token,
                "expiration": credentials.expiration.to_string(),
            }))
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
