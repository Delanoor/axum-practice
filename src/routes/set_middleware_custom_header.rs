use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("x-message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extension = request.extensions_mut();

    extension.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}
