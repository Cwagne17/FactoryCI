use std::str::FromStr;

use actix_web::{
    delete, get, http,
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::api::{database, error::ApiError};

#[derive(Deserialize)]
struct GetProjectsQuery {
    id: Option<String>,
}

#[get("/projects")]
async fn get_project(
    query: web::Query<GetProjectsQuery>,
    conn: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let projects;

    if let Some(id) = &query.id {
        // Unwrap the Option<String> to a String
        projects = database::projects::get_project_by_id(uuid::Uuid::from_str(&id).unwrap(), &conn)
            .await?;
    } else {
        projects = database::projects::get_all_projects(&conn).await?;
    }

    Ok(HttpResponse::Ok().json(projects))
}

#[delete("/projects/{id}")]
async fn delete_project(
    path: web::Path<String>,
    conn: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    database::projects::delete_project_by_id(
        uuid::Uuid::from_str(&path.into_inner()).unwrap(),
        &conn,
    )
    .await?;
    Ok(HttpResponse::Ok()
        .status(http::StatusCode::NO_CONTENT)
        .finish())
}
