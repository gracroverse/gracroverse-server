use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use crate::model::activitypub::ActivityStream;

pub fn router() -> Router {
    Router::new().route("/users/:username", get(get_user_object))
}

pub(crate) async fn get_user_object(username: String) -> Result<ActivityStream, StatusCode> {
    todo!()
}
