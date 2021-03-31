use actix_web::{web, HttpResponse};
use sqlx::types::Uuid;
use sqlx::PgPool;
use tera::{Context, Tera};
use wkhtmltopdf::{Orientation, PdfApplication, Size};
// use wkhtmltopdf::{Orientation, PdfApplication, Size};

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectReportView {
    pub id: Uuid,
    pub project_name: String,
    pub floor_plan_name: String,
    pub image_url: String,
    pub issue_name: String,
    pub issue_description: String,
    pub issue_location: String,
    pub filename: String,
    pub file_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewReport {
    pub project_name: String,
    pub floor_plan_room: String,
    pub floor_plan_url: String,
    pub issues: Vec<NewReportIssue>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NewReportIssue {
    pub name: String,
    pub description: String,
    pub location: String,
    pub url: String,
}

pub async fn create_overall_report(
    pool: web::Data<PgPool>,
    tmpl: web::Data<Tera>,
    project_id: web::Path<String>,
) -> Result<HttpResponse, HttpResponse> {
    let project_id = Uuid::parse_str(project_id.as_ref()).map_err(|e| {
        log::error!("create_overall_report: parse Uuid err from web::Path {}", e);
        HttpResponse::BadRequest().finish()
    })?;

    log::info!("Parsing incoming project id {:?}", project_id);

    let project_report_view = find_project_report_view(&pool, project_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    log::info!("Executed query {:?}", project_report_view);

    // Create Vec<Issue> for add to template
    let mut issues: Vec<NewReportIssue> = Vec::new();
    for it in project_report_view.clone() {
        issues.push(NewReportIssue {
            name: it.issue_name,
            description: it.issue_description,
            location: it.issue_location,
            url: it.file_url,
        })
    }

    // We know all of the project report view contains the same project id
    let new_report = NewReport {
        project_name: project_report_view.get(0).unwrap().project_name.clone(),
        floor_plan_room: project_report_view.get(0).unwrap().floor_plan_name.clone(),
        floor_plan_url: project_report_view.get(0).unwrap().image_url.clone(),
        issues,
    };
    log::info!("new_report {:?}", new_report);

    let mut context = Context::new();
    context.insert("project_name", &new_report.project_name);
    context.insert("floor_plan_room", &new_report.floor_plan_room);
    context.insert("floor_plan_url", &new_report.floor_plan_url);
    context.insert("issues", &new_report.issues);

    log::info!("Report context: {:?}", context);

    // let context = Context::from_serialize(new_report).map_err(|e| {
    //     log::error!("Err when serialize report: {}", e);
    //     HttpResponse::InternalServerError().finish()
    // })?;

    let html = tmpl.render("index.html", &context).map_err(|e| {
        log::error!("Getting error: {} from tera", e);
        HttpResponse::InternalServerError().finish()
    })?;

    log::info!("Converting: template: {}", html);

    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");

    let mut builder = pdf_app.builder();
    builder
        .orientation(Orientation::Landscape)
        .margin(Size::Millimeters(12))
        .title("Overall report");
    let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
    let printed_pdf = output
        .save("static/basic.pdf")
        .expect("Failed to save basic.pdf");
    log::info!(
        "Reading all basic.pdf {:?} to  static/overall_report.pdf",
        printed_pdf
    );

    // let mut resp = NamedFile::from_file(result, "../../static/overall_report.pdf")
    //     .map_err(|_| HttpResponse::InternalServerError().finish())?;
    // // let mut buffer = Vec::new();
    // // result.read_to_end(&mut buffer);
    // let cd: ContentDisposition = ContentDisposition {
    //     disposition: DispositionType::Attachment,
    //     parameters: vec![DispositionParam::Filename(String::from(
    //         "overall_report.pdf",
    //     ))],
    // };

    // let mut buffer = vec![];
    // printed_pdf.read_to_end(&mut buffer).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
    // Ok(HttpResponse::Ok().body(resp.set_content_disposition(cd).into_response(&req).into_body()))
}

pub async fn find_project_report_view(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<ProjectReportView>, sqlx::Error> {
    const QUERY: &str = "SELECT * FROM project_report_view WHERE id = $1";
    let project_report_view = sqlx::query_as::<_, ProjectReportView>(QUERY)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Query Error {:?}", e);
            e
        })?;
    Ok(project_report_view)
}

// async fn create_report_by_floor_plan_id() -> impl Responder {
//     let html = r#"
//     "<!DOCTYPE html>
//     <html lang="en">
//     <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
//    "#;
//
//     let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
//
//     let mut builder = pdf_app.builder();
//     builder
//         .orientation(Orientation::Landscape)
//         .margin(Size::Millimeters(12))
//         .title("Rust website");
//     let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
//     let _ = output
//         .save("static/basic.pdf")
//         .expect("Failed to save basic.pdf");
//     HttpResponse::Ok().finish()
// }
//
// async fn preview_report_by_floor_plan_id() -> impl Responder {
//     let html = r#"
//     "<!DOCTYPE html>
//     <html lang="en">
//     <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
//    "#;
//
//     let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
//
//     let mut builder = pdf_app.builder();
//     builder
//         .orientation(Orientation::Landscape)
//         .margin(Size::Millimeters(12))
//         .title("Rust website");
//     let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
//     let _ = output
//         .save("static/basic.pdf")
//         .expect("Failed to save basic.pdf");
//     HttpResponse::Ok().finish()
// }
//
// async fn preview_report_by_project_id() -> impl Responder {
//     let html = r#"
//     "<!DOCTYPE html>
//     <html lang="en">
//     <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
//    "#;
//
//     let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
//
//     let mut builder = pdf_app.builder();
//     builder
//         .orientation(Orientation::Landscape)
//         .margin(Size::Millimeters(12))
//         .title("Rust website");
//     let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
//     let _ = output
//         .save("static/basic.pdf")
//         .expect("Failed to save basic.pdf");
//     HttpResponse::Ok().finish()
// }
