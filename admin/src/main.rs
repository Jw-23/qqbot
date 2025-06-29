use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use env_logger;
use log::info;

mod handlers;
mod models;
mod services;

use handlers::*;
use qqbot_core::config::get_db;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    // 初始化数据库连接
    get_db().await;
    
    info!("启动管理后台服务器 http://localhost:8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/students")
                            .route("", web::get().to(student_handler::list_students))
                            .route("", web::post().to(student_handler::create_student))
                            .route("/{id}", web::get().to(student_handler::get_student))
                            .route("/{id}", web::put().to(student_handler::update_student))
                            .route("/{id}", web::delete().to(student_handler::delete_student))
                            .route("/import", web::post().to(student_handler::import_students))
                            .route("/export", web::get().to(student_handler::export_students))
                            .route("/bulk-message", web::post().to(student_handler::bulk_message))
                    )
                    .service(
                        web::scope("/grades")
                            .route("", web::get().to(grade_handler::list_grades))
                            .route("", web::post().to(grade_handler::create_grade))
                            .route("/{id}", web::get().to(grade_handler::get_grade))
                            .route("/{id}", web::put().to(grade_handler::update_grade))
                            .route("/{id}", web::delete().to(grade_handler::delete_grade))
                            .route("/student/{student_id}", web::get().to(grade_handler::get_grades_by_student))
                    )
                    .service(
                        web::scope("/config")
                            .route("", web::get().to(config_handler::get_config))
                            .route("", web::put().to(config_handler::update_config))
                    )
            )
            .service(Files::new("/", "./admin/frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}
