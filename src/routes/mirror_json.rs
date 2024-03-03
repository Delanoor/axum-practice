use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_server: String,
}

pub async fn mirror_json(Json(body): Json<MirrorJson>) -> (StatusCode, Json<MirrorJsonResponse>) {
    let res = MirrorJsonResponse {
        message: body.message,
        message_server: StatusCode::OK.to_string(),
    };

    (StatusCode::ACCEPTED, Json(res))
}
