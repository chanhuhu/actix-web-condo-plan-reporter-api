use actix_web::{web, HttpResponse};
use sqlx::types::Uuid;
use sqlx::PgPool;
use tera::{Context, Tera};
// use wkhtmltopdf::{Orientation, PdfApplication, Size};

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Report {
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

pub async fn create_overall_report(
    pool: web::Data<PgPool>,
    tmpl: web::Data<Tera>,
    project_id: web::Path<String>,
) -> Result<HttpResponse, HttpResponse> {
    let project_id = Uuid::parse_str(project_id.as_ref()).expect("Failed to parse Uuid");

    log::info!("Parsing incoming project id {:?}", project_id);

    let report = find_project_report_view(&pool, project_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    log::info!("Executed query {:?}", report);

    let mut context = Context::new();
    context.insert("projects", &report);

    let result = tmpl
        .render("index.html", &context)
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    // let result = tmpl
    //     .render("index.html", &context)
    //     .map_err(|_| HttpResponse::InternalServerError().finish())?;
    // log::info!("rendered template result: {:?}", result);

    // let html = format!(
    //     r#"
    //  "<!DOCTYPE html>
    //  <html lang="en">
    //  <meta charset="UTF-8">
    //  <body>
    //     <div class="">
    //         <h1>ชื่อโปรเจค: {}</h1>
    //         <h2>ชั้น: {}</h2>
    //         <img style="width='100';height='100'" src="{}">
    //         <ul>
    //             <li>รายการแก้ไข{}:{}:{}</li>
    //         </ul>
    //         <img style="width='100';height='100'" src="{}">
    //         <p>{}</p>
    //         <a href="http://localhost:8000/static/basic.pdf">ดาว์นโหลด</a>
    //
    //         </div>
    // </body>
    // </html>"
    // "#,
    //     report[0].project_name,
    //     report[0].floor_plan_name,
    //     report[0].image_url,
    //     report[0].issue_name,
    //     report[0].issue_description,
    //     report[0].issue_location,
    //     report[0].filename,
    //     report[0].file_url,
    // );

    // let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    //
    // let mut builder = pdf_app.builder();
    // builder
    //     .orientation(Orientation::Landscape)
    //     .margin(Size::Millimeters(12))
    //     .title("Overall report");
    // let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
    // let mut printed_pdf = output
    //     .save("static/basic.pdf")
    //     .expect("Failed to save basic.pdf");
    // log::info!(
    //     "Reading all basic.pdf {:?} to  static/overall_report.pdf",
    //     printed_pdf
    // );

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
    Ok(HttpResponse::Ok().content_type("text/html").body(result))
    // Ok(HttpResponse::Ok().body(resp.set_content_disposition(cd).into_response(&req).into_body()))
}

pub async fn find_project_report_view(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Report>, sqlx::Error> {
    const QUERY: &str = "SELECT * FROM project_report_view WHERE id = $1";
    let report = sqlx::query_as::<_, Report>(QUERY)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Query Error {:?}", e);
            e
        })?;
    Ok(report)
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
