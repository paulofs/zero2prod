use axum::extract::Query;
use hyper::StatusCode;

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm(_parameters: Query<Parameters>) -> StatusCode {
    StatusCode::OK
}
#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}
