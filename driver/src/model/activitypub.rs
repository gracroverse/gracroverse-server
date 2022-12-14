use axum::http::{header, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use bytes::{BufMut as _, BytesMut};
use serde::Serialize;

/// Newtype for JSON which represents JSON-LD ActivityStream2 objects.
///
/// Implements [`IntoResponse`], so we can return this from Axum routes
/// and have `Content-Type` and friends be handled automatically.
pub struct ActivityStream<T: Serialize = serde_json::Value>(pub T);
impl<T: Serialize> IntoResponse for ActivityStream<T> {
    fn into_response(self) -> axum::response::Response {
        let mut buf = BytesMut::new().writer();
        match serde_json::to_writer(&mut buf, &self.0) {
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/activity+json"),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain; charset=utf-8"),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}
