use crate::consts::FILE_STORAGE_KEY_FOLDER;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures::TryStreamExt;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::io::Write;
use std::path::{Path, PathBuf};

/// parameters for route /{project_id}/floor_plans
#[derive(serde::Deserialize)]
pub struct Parameters {
    project_id: String,
}

#[derive(serde::Deserialize)]
pub struct NewFloorPlan {
    pub id: Uuid,
    pub name: String,
    pub image_url: String,
}

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct FloorPlan {
    pub id: Uuid,
    pub name: String,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn change_file_name_with_uuid(path: impl AsRef<Path>, uuid: Uuid) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    let name = format!("{}", uuid);
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(|s| s.to_str())
}

pub fn get_stem_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).file_stem().and_then(|s| s.to_str())
}

pub async fn create_floor_plan(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;
        let filename = sanitize_filename::sanitize(&filename);
        // let file_extension = get_extension_from_filename(&filename).unwrap();
        let file_stem = get_stem_from_filename(filename.as_ref()).unwrap();
        let floor_plan_id = Uuid::new_v4();
        let base_filepath = Path::new(".").join(FILE_STORAGE_KEY_FOLDER).join(&filename);
        let filepath = change_file_name_with_uuid(base_filepath, floor_plan_id);
        // let image_url = format!(
        //     "{}.{}",
        //     file_url("http://localhost:8000", floor_plan_id),
        //     file_extension
        // );
        // converting to url for showing the floor plan img
        let image_url = filepath.clone();
        let image_url = format!(
            "{}{}",
            "http://localhost:8000/",
            image_url
                .strip_prefix("./")
                .unwrap()
                .to_string_lossy()
                .to_string()
        );
        log::info!("Image url: {}", image_url);

        let new_flor_plan = NewFloorPlan {
            id: floor_plan_id,
            name: file_stem.to_string(),
            image_url,
        };
        let project_id =
            Uuid::parse_str(parameters.project_id.as_ref()).expect("Failed to parsed Uuid");

        insert_floor_plan(&pool, &new_flor_plan, project_id)
            .await
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath).unwrap()).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || {
                f.write_all(&chunk)
                    .map(|_| f)
                    .expect("Failed to write files")
            })
            .await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub async fn list_floor_plans(
    pool: web::Data<PgPool>,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let project_id = Uuid::parse_str(parameters.project_id.as_ref()).unwrap();
    let floor_plans = find_floor_plans_by_project_id(&pool, project_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().json(floor_plans))
}

pub async fn get_floor_plan_details(
    pool: web::Data<PgPool>,
    floor_plan_id: web::Path<String>,
) -> Result<HttpResponse, HttpResponse> {
    let floor_plan_id = Uuid::parse_str(&floor_plan_id).map_err(|e| {
        log::error!("Error parse Uuid {}", e);
        HttpResponse::BadRequest().finish()
    })?;
    let floor_plan = find_floor_plan(&pool, floor_plan_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    match floor_plan {
        Some(res) => Ok(HttpResponse::Ok().json(res)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

async fn find_floor_plan(
    pool: &PgPool,
    floor_plan_id: Uuid,
) -> Result<Option<FloorPlan>, sqlx::Error> {
    let floor_plan = sqlx::query_as::<_, FloorPlan>(r#"SELECT * FROM floor_plans WHERE id = $1"#)
        .bind(floor_plan_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to query {:?}", e);
            e
        })?;
    Ok(floor_plan)
}

async fn find_floor_plans_by_project_id(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<FloorPlan>, sqlx::Error> {
    log::info!(
        "Getting projects in the database by project_id {}",
        project_id
    );
    let floor_plans =
        sqlx::query_as::<_, FloorPlan>(r#"SELECT * FROM floor_plans WHERE project_id = $1"#)
            .bind(project_id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::error!("Failed to query {:?}", e);
                e
            })?;
    log::info!("Result from getting {:?}", floor_plans);
    Ok(floor_plans)
}

pub async fn insert_floor_plan(
    pool: &PgPool,
    input: &NewFloorPlan,
    project_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO floor_plans
    (id, project_id, name, image_url, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)"#,
        input.id,
        project_id,
        input.name,
        input.image_url,
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to query {}", e);
        e
    })?;
    Ok(())
}
