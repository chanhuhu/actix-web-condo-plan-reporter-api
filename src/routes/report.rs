use actix_web::{HttpResponse, Responder};
use wkhtmltopdf::{Orientation, PdfApplication, Size};

async fn create_report_by_project_id() -> impl Responder {
    let html = r#"
    "<!DOCTYPE html>
    <html lang="en">
    <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
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

async fn create_report_by_floor_plan_id() -> impl Responder {
    let html = r#"
    "<!DOCTYPE html>
    <html lang="en">
    <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
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

async fn preview_report_by_floor_plan_id() -> impl Responder {
    let html = r#"
    "<!DOCTYPE html>
    <html lang="en">
    <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
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

async fn preview_report_by_project_id() -> impl Responder {
    let html = r#"
    "<!DOCTYPE html>
    <html lang="en">
    <meta charset="UTF-8"><body><div class=""><h1>สวัสดั</h1><img src="https://www.rust-lang.org/logos/rust-logo-512x512.png"><h1>This is a Heading</h1><p>This is a paragraph.</p></div></body></html>"
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
