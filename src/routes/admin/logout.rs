use axum::response::{IntoResponse, Response};
use axum_sessions::extractors::WritableSession;

use crate::utils::see_other;

pub async fn log_out(mut session: WritableSession) -> Response {
    if session.get::<uuid::Uuid>("user_id").is_none() {
        see_other("/login")
    } else {
        session.destroy();
        (
            axum::http::StatusCode::SEE_OTHER,
            [
                (
                    axum::http::header::COOKIE,
                    "_flash=You have successfully logged out.",
                ),
                (axum::http::header::LOCATION, "/login"),
            ],
        )
            .into_response()
    }
}
