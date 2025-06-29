use actix_web::{web, HttpResponse, Result};
use crate::models::config::*;
use std::fs;
use std::io::Write;

pub async fn get_config() -> Result<HttpResponse> {
    match fs::read_to_string("config.dev.toml") {
        Ok(content) => {
            match toml::from_str::<ConfigDto>(&content) {
                Ok(config) => Ok(HttpResponse::Ok().json(config)),
                Err(e) => Ok(HttpResponse::InternalServerError()
                    .json(format!("配置解析错误: {}", e))),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError()
            .json(format!("读取配置文件错误: {}", e))),
    }
}

pub async fn update_config(req: web::Json<ConfigDto>) -> Result<HttpResponse> {
    match toml::to_string_pretty(&req.into_inner()) {
        Ok(toml_content) => {
            match fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("config.dev.toml") {
                Ok(mut file) => {
                    match file.write_all(toml_content.as_bytes()) {
                        Ok(_) => Ok(HttpResponse::Ok().json("配置更新成功")),
                        Err(e) => Ok(HttpResponse::InternalServerError()
                            .json(format!("写入配置文件错误: {}", e))),
                    }
                }
                Err(e) => Ok(HttpResponse::InternalServerError()
                    .json(format!("打开配置文件错误: {}", e))),
            }
        }
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(format!("配置序列化错误: {}", e))),
    }
}
