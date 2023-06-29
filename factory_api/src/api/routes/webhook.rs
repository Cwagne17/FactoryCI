use actix_web::{post, web::Json, HttpResponse};
use serde_json::Value;

use crate::api::error::ApiError;

#[post("/github/webhook")]
pub async fn github_webhook(payload: Json<Value>) -> Result<HttpResponse, ApiError> {
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    Ok(HttpResponse::Ok().finish())
}
