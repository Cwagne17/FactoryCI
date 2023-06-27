use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::{error::ApiError, projects::Project};

pub async fn get_projects(id: Uuid, conn: &PgPool) -> Result<Vec<Project>, ApiError> {
    let projects: Vec<Project> = sqlx::query!(
        r#"
        SELECT id, url, webhook_secret
        FROM projects
        WHERE id = $1
        "#,
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
