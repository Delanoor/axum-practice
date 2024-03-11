use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    url: String,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Im created!".to_string(),
        url: "https://github.com/delanoor".to_string(),
        username: "hwn".to_string(),
    };

    Json(data)
}
