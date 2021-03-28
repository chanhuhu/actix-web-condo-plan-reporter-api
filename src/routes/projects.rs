use crate::domain::{NewProject, ProjectName};
use actix_web::{web, HttpResponse};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::PgPool;

/// project input
#[derive(serde::Deserialize)]
pub struct ProjectInput {
    name: String,
}

/// parameters for route projects/{project_id}
#[derive(serde::Deserialize)]
pub struct Parameters {
    project_id: String,
}

/// entities::Project
#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn parse_project(input: ProjectInput) -> Result<NewProject, String> {
    let name = ProjectName::parse(input.name)?;
    Ok(NewProject { name })
}

pub async fn list_projects(pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let projects = find_projects(&pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().json(projects))
}

pub async fn get_project_details(
    pool: web::Data<PgPool>,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let project_id = Uuid::parse_str(parameters.project_id.as_ref()).expect("Failed to parse Uuid");
    let project = find_project(&pool, project_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().json(project))
}

pub async fn find_project(pool: &PgPool, project_id: Uuid) -> Result<Project, sqlx::Error> {
    match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(pool)
        .await
    {
        Ok(None) => Err(sqlx::Error::RowNotFound),
        Ok(Some(result)) => Ok(result),
        Err(error) => Err(error),
    }
}

pub async fn find_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(pool)
        .await
        .map_err(|e| e)?;
    Ok(projects)
}

pub async fn create_project(
    pool: web::Data<PgPool>,
    input: web::Json<ProjectInput>,
) -> Result<HttpResponse, HttpResponse> {
    let new_project = parse_project(input.0).map_err(|_| HttpResponse::BadRequest().finish())?;
    insert_project(&pool, &new_project)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn rename_project(
    pool: web::Data<PgPool>,
    input: web::Json<ProjectInput>,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let new_project = parse_project(input.0).map_err(|_| HttpResponse::BadRequest().finish())?;
    let project_id = Uuid::parse_str(parameters.project_id.as_ref()).expect("Failed to parse Uuid");
    update_project(&pool, &new_project, project_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn insert_project(pool: &PgPool, input: &NewProject) -> Result<(), sqlx::Error> {
    let project_id = Uuid::new_v4();
    // we use project name as a parent folder for flor plan images.
    // clean and validate input
    sqlx::query!(
        r#"INSERT INTO projects
    (id, name, created_at, updated_at)
    VALUES ($1, $2, $3, $4)"#,
        project_id,
        input.name.as_ref(),
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| e)?;
    Ok(())
}

pub async fn update_project(
    pool: &PgPool,
    input: &NewProject,
    project_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE projects SET
    name = $1, updated_at = $2
    WHERE id = $3 "#,
        input.name.as_ref(),
        Utc::now(),
        project_id,
    )
    .execute(pool)
    .await
    .map_err(|e| e)?;
    Ok(())
}
