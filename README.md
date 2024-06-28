# AWS Temporary Credentials Generator

_A Rust server to generate temporary credentials valid for 15 minutes using AWS STS._

## Usage

Set the environment variables below in the `.env` file:

```toml
AWS_ACCESS_KEY_ID=********************
AWS_SECRET_ACCESS_KEY=****************************************
AWS_DEFAULT_REGION=ap-south-1
```

Run `cargo run --release` to generate temporary credentials.
