use std::str::FromStr;

use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::api::{database, error::ApiError};

#[derive(Deserialize)]
struct GetProjectsQuery {
    id: String,
}

#[get("/projects")]
async fn get_projects(
    query: web::Query<GetProjectsQuery>,
    conn: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    println!(
        "get_projects: {:?}",
        uuid::Uuid::from_str(&query.id).unwrap()
    );
    let projects: Vec<crate::api::projects::Project> =
        database::projects::get_projects(uuid::Uuid::from_str(&query.id).unwrap(), &conn).await?;
    Ok(HttpResponse::Ok().json(projects))
}
