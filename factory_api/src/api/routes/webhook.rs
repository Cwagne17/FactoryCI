use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use serde_json::Value;

use crate::api::error::ApiError;

pub enum GithubEvents {
    /// This event occurs when there is activity relating to which repositories a GitHub App installation can access.
    /// All GitHub Apps receive this event by default. You cannot manually subscribe to this event.
    InstallationRepositories,
    /// This event occurs when there is activity relating to a GitHub App installation.
    /// All GitHub Apps receive this event by default. You cannot manually subscribe to this event
    Installation,
    /// This event occurs when a commit or tag is pushed.
    Push,
    /// This event occurs when there is activity on a pull request.
    PullRequest,
}

#[post("/github/webhook")]
pub async fn github_webhook(
    payload: Json<Value>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    println!("{:?}", req.headers());
    Ok(HttpResponse::Ok().finish())
}
