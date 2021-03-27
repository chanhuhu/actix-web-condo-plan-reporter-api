use actix_web::dev::Server;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use wkhtmltopdf::{Orientation, PdfApplication, Size};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest())?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| HttpResponse::BadRequest())?;
        let filepath = format!("./static/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath).unwrap()).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || {
                f.write_all(&data)
                    .map(|_| f)
                    .expect("Failed to write files")
            })
            .await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn generate_report() -> impl Responder {
    let html = r#"
   <html><body><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"></body></html>
   "#;

    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");

    let mut builder = pdf_app.builder();
    builder
        .orientation(Orientation::Landscape)
        .margin(Size::Millimeters(12))
        .title("Rust website");
    let mut output = builder.build_from_html(&html).expect("Failed to build pdf");
    let _ = output
        .save("static/basic.pdf")
        .expect("Failed to save basic.pdf");
    HttpResponse::Ok().finish()
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    std::fs::create_dir_all("./static").unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/upload")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(generate_report))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
