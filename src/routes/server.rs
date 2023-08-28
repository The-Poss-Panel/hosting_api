use crate::State;
use actix_web::{get, post, web, HttpResponse, Responder};
use entity::{prelude::Servers, servers};
use hosting_types::Response;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Form {
    name: String,
}

#[get("/server/{id}")]
pub async fn find(state: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let server = Servers::find_by_id(id)
        .into_json()
        .one(&state.db)
        .await
        .unwrap();

    if let Some(server) = server {
        HttpResponse::Ok().json(&server)
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}

#[post("/server/{id}")]
pub async fn modify(
    state: web::Data<State>,
    path: web::Path<i32>,
    form: web::Json<Form>,
) -> impl Responder {
    let id = path.into_inner();

    if Servers::update(servers::ActiveModel {
        name: Set(form.name.to_string()),
        ..Default::default()
    })
    .filter(servers::Column::Id.eq(id))
    .exec(&state.db)
    .await
    .is_ok()
    {
        HttpResponse::Ok().json(Response {
            error: false,
            message: "The server has been updated".into(),
        })
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}
