use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::todo::{Todo, NewTodo ,TodoList};

pub fn get_all(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(TodoList::get_all())
}

pub fn create(new_todo: web::Json<NewTodo>) -> Result<HttpResponse, HttpResponse> {
    new_todo
        .create()
        .map(|todo| HttpResponse::Created().json(todo))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn get_by_id(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Todo::get_by_id(&id)
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn delete_by_id(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Todo::delete_by_id(&id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn update_by_id(id: web::Path<i32>, new_todo: web::Json<NewTodo>) -> Result<HttpResponse, HttpResponse> {
    Todo::update_by_id(&id, &new_todo).unwrap();
    get_by_id(id)
}