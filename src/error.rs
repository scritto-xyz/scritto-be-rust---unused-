pub mod error {
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use serde_json::json;

    #[derive(Debug)]
    pub enum AppError {
        InvalidToken,
        WrongCredential,
        MissingCredential,
        TokenCreation,
        InternalServerError,
        UserDoesNotExist,
        UserAlreadyExists,
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> axum::response::Response {
            let (status, err_msg) = match self {
                Self::InternalServerError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "an internal server error occurred",
                    ),
                Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
                Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
                Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create token"),
                Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credentials"),
                Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
                Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already exists"),
            };
            (status, Json(json!({"error": err_msg}))).into_response()
        }
    }
}
