use actix_web::{web, HttpResponse, Result};
use qqbot_core::{
    config::get_db,
    models::grade::{self, Entity as Grade},
    service::grade_service,
};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, ColumnTrait, QueryFilter};
use crate::models::grade::*;
use crate::handlers::student_handler::{ListQuery, ListResponse};

pub async fn list_grades(query: web::Query<ListQuery>) -> Result<HttpResponse> {
    let db = get_db().await;
    
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    let paginator = Grade::find()
        .order_by_asc(grade::Column::Id)
        .paginate(db.as_ref(), limit as u64);
    
    let total = paginator.num_items().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
    })?;
    
    let grades = paginator
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
        })?;
    
    let grade_dtos: Vec<GradeDto> = grades
        .into_iter()
        .map(|g| GradeDto {
            id: Some(g.id),
            student_name: g.student_name,
            exam_name: g.exam_name,
            course_id: g.course_id,
            course_seq: g.course_seq,
            student_id: g.student_id,
            score: g.score,
            category: g.category,
        })
        .collect();
    
    let response = ListResponse {
        data: grade_dtos,
        total,
        page,
        limit,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_grade(path: web::Path<i64>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db = get_db().await;
    
    match Grade::find_by_id(id).one(db.as_ref()).await {
        Ok(Some(grade)) => {
            let dto = GradeDto {
                id: Some(grade.id),
                student_name: grade.student_name,
                exam_name: grade.exam_name,
                course_id: grade.course_id,
                course_seq: grade.course_seq,
                student_id: grade.student_id,
                score: grade.score,
                category: grade.category,
            };
            Ok(HttpResponse::Ok().json(dto))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("成绩记录不存在")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("数据库错误: {}", e))),
    }
}

pub async fn create_grade(req: web::Json<CreateGradeRequest>) -> Result<HttpResponse> {
    match grade_service::create_grade(
        &req.student_name,
        &req.exam_name,
        req.course_id,
        req.course_seq,
        req.student_id,
        req.score,
        &req.category,
    ).await {
        Ok(grade) => {
            let dto = GradeDto {
                id: Some(grade.id),
                student_name: grade.student_name,
                exam_name: grade.exam_name,
                course_id: grade.course_id,
                course_seq: grade.course_seq,
                student_id: grade.student_id,
                score: grade.score,
                category: grade.category,
            };
            Ok(HttpResponse::Created().json(dto))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("创建失败: {}", e))),
    }
}

pub async fn update_grade(
    path: web::Path<i64>,
    req: web::Json<UpdateGradeRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    match grade_service::update_grade_by_id(
        id,
        req.student_name.clone(),
        req.exam_name.clone(),
        req.course_id,
        req.course_seq,
        req.student_id,
        req.score,
        req.category.clone(),
    ).await {
        Ok(grade) => {
            let dto = GradeDto {
                id: Some(grade.id),
                student_name: grade.student_name,
                exam_name: grade.exam_name,
                course_id: grade.course_id,
                course_seq: grade.course_seq,
                student_id: grade.student_id,
                score: grade.score,
                category: grade.category,
            };
            Ok(HttpResponse::Ok().json(dto))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("更新失败: {}", e))),
    }
}

pub async fn delete_grade(path: web::Path<i64>) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    match grade_service::delete_grade_by_id(id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("删除失败: {}", e))),
    }
}

pub async fn get_grades_by_student(path: web::Path<i64>) -> Result<HttpResponse> {
    let student_id = path.into_inner();
    let db = get_db().await;
    
    let grades = Grade::find()
        .filter(grade::Column::StudentId.eq(student_id))
        .order_by_asc(grade::Column::Id)
        .all(db.as_ref())
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("数据库错误: {}", e))
        })?;
    
    let grade_dtos: Vec<GradeDto> = grades
        .into_iter()
        .map(|g| GradeDto {
            id: Some(g.id),
            student_name: g.student_name,
            exam_name: g.exam_name,
            course_id: g.course_id,
            course_seq: g.course_seq,
            student_id: g.student_id,
            score: g.score,
            category: g.category,
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(grade_dtos))
}
