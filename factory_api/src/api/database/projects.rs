use sqlx::PgPool;
use uuid::Uuid;

use crate::api::{error::ApiError, projects::Project};

pub async fn get_project_by_id(id: Uuid, conn: &PgPool) -> Result<Vec<Project>, ApiError> {
    let projects: Vec<Project> = sqlx::query!(
        "SELECT id, url, webhook_secret FROM projects WHERE id = $1",
        id
    )
    .fetch_all(conn)
    .await
    .context("Failed to query projects")
    .map_err(ApiError::Database)?
    .iter()
    .map(|row| {
        Project::new(
            row.id.to_string(),
            row.url.clone(),
            row.webhook_secret.clone(),
        )
    })
    .collect();
    Ok(projects)
}

pub async fn get_all_projects(conn: &PgPool) -> Result<Vec<Project>, ApiError> {
    let projects: Vec<Project> = sqlx::query!("SELECT id, url, webhook_secret FROM projects")
        .fetch_all(conn)
        .await
        .context("Failed to query projects")
        .map_err(ApiError::Database)?
        .iter()
        .map(|row| {
            Project::new(
                row.id.to_string(),
                row.url.clone(),
                row.webhook_secret.clone(),
            )
        })
        .collect();
    Ok(projects)
}

pub async fn insert_project(conn: &PgPool, project: Project) -> Result<(), ApiError> {
    sqlx::query!(
        "INSERT INTO projects (id, url, webhook_secret) VALUES ($1, $2, $3)",
        Uuid::parse_str(&project.id()).unwrap(),
        project.url(),
        project.webhook_secret()
    )
    .execute(conn)
    .await
    .context("Failed to insert new project")
    .map_err(ApiError::Database)?;
    Ok(())
}

pub async fn delete_project_by_id(id: Uuid, conn: &PgPool) -> Result<(), ApiError> {
    sqlx::query!("DELETE FROM projects WHERE id = $1", id)
        .execute(conn)
        .await
        .context("Failed to delete project")
        .map_err(ApiError::Database)?;
    Ok(())
}
