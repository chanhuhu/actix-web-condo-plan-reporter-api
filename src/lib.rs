pub mod configuration;
pub mod consts;
pub mod domain;
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
//
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
//             .route("/health_check", web::get().to(health_check))
//             .route("/", web::get().to(create_report))
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }
