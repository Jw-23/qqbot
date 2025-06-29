use actix_web::{web, HttpResponse, Result};
use qqbot_core::{
    config::get_db,
    models::student::{self, Entity as Student},
    service::student_service,
};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use crate::models::student::*;
use csv::Writer;
use std::io::Cursor;

pub async fn list_students(query: web::Query<ListQuery>) -> Result<HttpResponse> {
    let db = get_db().await;
    
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    
    let paginator = Student::find()
        .order_by_asc(student::Column::Id)
        .paginate(db.as_ref(), limit as u64);
    
    let total = paginator.num_items().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
    })?;
    
    let students = paginator
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
        })?;
    
    let student_dtos: Vec<StudentDto> = students
        .into_iter()
        .map(|s| StudentDto {
            id: Some(s.id),
            student_id: s.student_id,
            name: s.name,
            qq_number: s.qq_number,
            group_id: s.group_id,
            created_at: Some(s.created_at.with_timezone(&chrono::Utc)),
            updated_at: Some(s.updated_at.with_timezone(&chrono::Utc)),
        })
        .collect();
    
    let response = ListResponse {
        data: student_dtos,
        total,
        page,
        limit,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_student(path: web::Path<i64>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db = get_db().await;
    
    match Student::find_by_id(id).one(db.as_ref()).await {
        Ok(Some(student)) => {
            let dto = StudentDto {
                id: Some(student.id),
                student_id: student.student_id,
                name: student.name,
                qq_number: student.qq_number,
                group_id: student.group_id,
                created_at: Some(student.created_at.with_timezone(&chrono::Utc)),
                updated_at: Some(student.updated_at.with_timezone(&chrono::Utc)),
            };
            Ok(HttpResponse::Ok().json(dto))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("学生不存在")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("数据库错误: {}", e))),
    }
}

pub async fn create_student(req: web::Json<CreateStudentRequest>) -> Result<HttpResponse> {
    match student_service::create_student(
        req.student_id,
        &req.name,
        req.qq_number,
        req.group_id,
    ).await {
        Ok(student) => {
            let dto = StudentDto {
                id: Some(student.id),
                student_id: student.student_id,
                name: student.name,
                qq_number: student.qq_number,
                group_id: student.group_id,
                created_at: Some(student.created_at.with_timezone(&chrono::Utc)),
                updated_at: Some(student.updated_at.with_timezone(&chrono::Utc)),
            };
            Ok(HttpResponse::Created().json(dto))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("创建失败: {}", e))),
    }
}

pub async fn update_student(
    path: web::Path<i64>,
    req: web::Json<UpdateStudentRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    match student_service::update_student_by_id(
        id,
        req.student_id,
        req.name.clone(),
        req.qq_number,
        req.group_id,
    ).await {
        Ok(student) => {
            let dto = StudentDto {
                id: Some(student.id),
                student_id: student.student_id,
                name: student.name,
                qq_number: student.qq_number,
                group_id: student.group_id,
                created_at: Some(student.created_at.with_timezone(&chrono::Utc)),
                updated_at: Some(student.updated_at.with_timezone(&chrono::Utc)),
            };
            Ok(HttpResponse::Ok().json(dto))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("更新失败: {}", e))),
    }
}

pub async fn delete_student(path: web::Path<i64>) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    match student_service::delete_student_by_id(id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("删除失败: {}", e))),
    }
}

pub async fn import_students(req: web::Json<ImportStudentsRequest>) -> Result<HttpResponse> {
    let mut success_count = 0;
    let mut errors = Vec::new();
    
    for (index, student_req) in req.students.iter().enumerate() {
        match student_service::create_student(
            student_req.student_id,
            &student_req.name,
            student_req.qq_number,
            student_req.group_id,
        ).await {
            Ok(_) => success_count += 1,
            Err(e) => errors.push(format!("第{}行: {}", index + 1, e)),
        }
    }
    
    let response = ImportResponse {
        success_count,
        total_count: req.students.len(),
        errors,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn export_students() -> Result<HttpResponse> {
    let db = get_db().await;
    
    let students = Student::find().all(db.as_ref()).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
    })?;
    
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));
    wtr.write_record(&["学号", "姓名", "QQ号", "群号"]).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("CSV写入错误: {}", e))
    })?;
    
    for student in students {
        wtr.write_record(&[
            student.student_id.to_string(),
            student.name,
            student.qq_number.to_string(),
            student.group_id.to_string(),
        ]).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("CSV写入错误: {}", e))
        })?;
    }
    
    let data = wtr.into_inner().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("CSV生成错误: {}", e))
    })?.into_inner();
    
    Ok(HttpResponse::Ok()
        .content_type("text/csv; charset=utf-8")
        .insert_header(("Content-Disposition", "attachment; filename=students.csv"))
        .body(data))
}

pub async fn bulk_message(req: web::Json<BulkMessageRequest>) -> Result<HttpResponse> {
    // 这里需要集成QQ机器人发送消息的功能
    
    // 暂时返回成功响应
    let response = BulkMessageResponse {
        success_count: req.student_ids.len(),
        message: "消息发送功能需要集成QQ机器人".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[derive(serde::Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(serde::Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

#[derive(serde::Serialize)]
pub struct ImportResponse {
    pub success_count: usize,
    pub total_count: usize,
    pub errors: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct BulkMessageResponse {
    pub success_count: usize,
    pub message: String,
}
