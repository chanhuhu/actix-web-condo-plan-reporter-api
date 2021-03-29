use crate::routes::{
    create_floor_plan, create_issue, create_project, get_floor_plan_details, get_project_details,
    health_check, index, list_floor_plans, list_issue, list_projects, rename_project,
};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/api/v1")
                    // Endpoint: /projects/{project_id}/floor_plans
                    .service(
                        web::scope("/projects")
                            .service(
                                web::resource("")
                                    .route(web::get().to(list_projects))
                                    .route(web::post().to(create_project)),
                            )
                            .service(
                                web::scope("/{project_id}")
                                    .service(
                                        web::resource("")
                                            .route(web::get().to(get_project_details))
                                            .route(web::put().to(rename_project)),
                                    )
                                    .service(
                                        web::resource("/floor_plans")
                                            .route(web::post().to(create_floor_plan))
                                            .route(web::get().to(list_floor_plans)),
                                    ),
                            ),
                    )
                    // Endpoint: /floor_plans/{floor_plan_id}/issues
                    .service(
                        web::scope("/floor_plans").service(
                            web::scope("/{floor_plan_id}")
                                .service(
                                    web::resource("").route(web::get().to(get_floor_plan_details)),
                                )
                                .service(
                                    web::resource("/issues")
                                        .route(web::post().to(create_issue))
                                        .route(web::get().to(list_issue)),
                                ),
                        ),
                    ),
            )
            .service(Files::new("/static", "./static").show_files_listing())
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
