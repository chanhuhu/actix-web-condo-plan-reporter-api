use crate::consts::FILE_STORAGE_KEY_FOLDER;
use crate::domain::{NewFile, NewIssue};
use crate::routes::{find_projects, get_extension_from_filename, get_stem_from_filename};
use actix_multipart::{Field, Multipart};
use actix_web::web::Bytes;
use actix_web::{web, Error, HttpResponse};
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{PgPool, Postgres, Transaction};
use std::borrow::BorrowMut;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct Issue {
    pub id: Uuid,
    pub floor_plan_id: Uuid,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(serde::Deserialize)]
pub struct Parameters {
    pub floor_plan_id: String,
}

pub fn file_url(base_url: &str, file_id: Uuid) -> String {
    format!("{}/{}/{}", base_url, FILE_STORAGE_KEY_FOLDER, file_id)
}

pub fn parse_file_path_with_file_id(file_id: Uuid) -> PathBuf {
    let file_path = format!("{}/{}", FILE_STORAGE_KEY_FOLDER, file_id);
    let file_path = Path::new(".").join(file_path);
    file_path
}

pub fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="" method="post" enctype="multipart/form-data" id="myForm" >
                <input type="text"  id="name" name="text" value="test_text"/>    
                <input type="text"  id="description" name="text" value="123123"/>    
                <input type="text"  id="location" name="text" value="123123"/>    
                
                <input type="button" value="Submit" onclick="myFunction()"></button>
            </form>
            <input type="file" multiple name="file" id="myFile"/>
        </body>
        <script>
        function myFunction(){
            var myForm = document.getElementById('myForm');
            var myFile = document.getElementById('myFile');
    
            let formData = new FormData();
            const obj = {
                name: document.getElementById('name').value,
                description: document.getElementById('description').value,
                location: document.getElementById('location').value
            };
            const json = JSON.stringify(obj);
            console.log(obj);
            console.log(json);
    
            
            formData.append("data", json);
            formData.append("myFile", myFile.files[0]);
    
            var request = new XMLHttpRequest();
            request.open("POST", "api/v1/floor_plans/6e727c59-7b32-43d0-a87a-160e78f93f20/issues");
            request.send(formData);
        }
        
        
        </script>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn split_payload(payload: &mut Multipart) -> Result<(bytes::Bytes, Vec<NewFile>), Error> {
    let mut data = Bytes::new();
    let mut files: Vec<NewFile> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name == "data" {
            while let Some(chunk) = field.next().await {
                data = chunk.expect(" split_payload err chunk");
            }
        } else {
            match content_type.get_filename() {
                Some(filename) => {
                    let filename = sanitize_filename::sanitize(&filename);
                    let file_extension =
                        get_extension_from_filename(&filename).expect("Failed to parse ext");
                    let file_stem =
                        get_stem_from_filename(&filename).expect("Failed to parse stem");
                    let file_id = Uuid::new_v4();
                    let file_path = parse_file_path_with_file_id(file_id);
                    let file_path = format!(
                        "{}.{}",
                        file_path.to_string_lossy().to_string(),
                        file_extension
                    );
                    let file_url =
                        format!("{}.{}", file_url("localhost:8000", file_id), file_extension);
                    let new_file = NewFile {
                        id: file_id,
                        name: file_stem.to_string(),
                        url: file_url,
                    };
                    let mut f =
                        web::block(move || std::fs::File::create(&file_path).unwrap()).await?;
                    while let Some(chunk) = field.try_next().await? {
                        // filesystem operations are blocking, we have to use threadpool
                        f = web::block(move || {
                            f.write_all(&chunk)
                                .map(|_| f)
                                .expect("Failed to write files")
                        })
                        .await?;
                    }
                    files.push(new_file);
                }
                None => {
                    println!("file none");
                }
            }
        }
    }
    Ok((data, files))
}

pub async fn list_issue(
    pool: web::Data<PgPool>,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let floor_plan_id = Uuid::parse_str(parameters.floor_plan_id.as_ref())
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let issues = find_issues(&pool, floor_plan_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(issues))
}

pub async fn find_issues(pool: &PgPool, floor_plan_id: Uuid) -> Result<Vec<Issue>, sqlx::Error> {
    let issues = sqlx::query_as::<_, Issue>("SELECT * FROM issues WHERE floor_plan_id = $1")
        .bind(floor_plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Failed to query {:?}", e);
            e
        })?;
    Ok(issues)
}

pub async fn create_issue(
    pool: web::Data<PgPool>,
    mut multipart: Multipart,
    parameters: web::Path<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    log::info!(
        "Adding new issue by floor id {:?}",
        parameters.floor_plan_id
    );
    let floor_plan_id = Uuid::parse_str(parameters.floor_plan_id.as_ref()).unwrap();
    let payload = split_payload(multipart.borrow_mut())
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    log::info!("Parsing data {:?}", payload.0);
    let new_issue: NewIssue =
        serde_json::from_slice(&payload.0).map_err(|_| HttpResponse::BadRequest().finish())?;
    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    let issue_id = insert_issue(&mut transaction, &new_issue, floor_plan_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;
    let new_file = payload.1.get(0).unwrap();
    insert_file(&mut transaction, &new_file, issue_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    transaction
        .commit()
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

async fn insert_file(
    transaction: &mut Transaction<'_, Postgres>,
    new_file: &NewFile,
    issue_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO files
    (id, issue_id, name, url,created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)"#,
        new_file.id,
        issue_id,
        new_file.name,
        new_file.url,
        Utc::now(),
        Utc::now()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        log::error!("Failed to query {}", e);
        e
    })?;
    Ok(())
}

async fn insert_issue(
    transaction: &mut Transaction<'_, Postgres>,
    new_issue: &NewIssue,
    floor_plan_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    log::info!(
        "Adding new_issue '{}' '{}' '{}'",
        new_issue.name,
        new_issue.description,
        new_issue.location
    );
    let issue_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO issues
    (id, floor_plan_id, name, description, location, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        issue_id,
        floor_plan_id,
        new_issue.name,
        new_issue.description,
        new_issue.location,
        Utc::now(),
        Utc::now()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        log::error!("Failed to query {}", e);
        e
    })?;
    Ok(issue_id)
}
