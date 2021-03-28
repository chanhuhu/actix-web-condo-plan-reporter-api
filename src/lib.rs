pub mod configuration;
pub mod routes;
pub mod startup;

// use actix_web::dev::Server;
// use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
// use std::net::TcpListener;
//
// use actix_cors::Cors;
// use actix_files::Files;
// use actix_multipart::Multipart;
// use futures::TryStreamExt;
// use std::io::Write;
// use wkhtmltopdf::{Orientation, PdfApplication, Size};

// pub async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
//
// async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     // iterate over multipart stream
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let content_type = field
//             .content_disposition()
//             .ok_or_else(|| HttpResponse::BadRequest().finish())?;
//
//         let filename = content_type
//             .get_filename()
//             .ok_or_else(|| HttpResponse::BadRequest().finish())?;
//         let filepath = format!("./static/{}", sanitize_filename::sanitize(&filename));
//
//         // File::create is blocking operation, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath).unwrap()).await?;
//
//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.try_next().await? {
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || {
//                 f.write_all(&chunk)
//                     .map(|_| f)
//                     .expect("Failed to write files")
//             })
//             .await?;
//         }
//     }
//     Ok(HttpResponse::Ok().into())
// }
//
// async fn create_report() -> impl Responder {
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
// async fn index() -> impl Responder {
//     let html = r#"<html>
//         <head><title>Upload Test</title></head>
//         <body>
//             <form target="/" method="post" enctype="multipart/form-data">
//                 <input type="file" multiple name="file"/>
//                 <button type="submit">Submit</button>
//             </form>
//         </body>
//     </html>"#;
//
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }
//
// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     std::fs::create_dir_all("./static")?;
//
//     let server = HttpServer::new(move || {
//         let cors = Cors::permissive();
//         App::new()
//             .wrap(cors)
//             // domain includes: /projects/{project_id}/floor_plans/{floor_plan_id}
//             // domain includes: /projects/{project_id}/floor_plans/create_report
//             // domain includes: /floor_plans/{floor_plan_id}/issues/{issue_id}
//             // domain includes: /floor_plans/{floor_plan_id}/create_report
//             // domain includes: /issues/{issue_id}/images/{image_id}
//             .service(
//                 web::resource("/upload")
//                     .route(web::get().to(index))
//                     .route(web::post().to(save_file)),
//             )
//             .service(Files::new("/images", "./static").show_files_listing())
//             .route("/health_check", web::get().to(health_check))
//             .route("/", web::get().to(create_report))
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }
