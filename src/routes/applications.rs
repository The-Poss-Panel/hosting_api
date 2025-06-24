use crate::State;
use actix_web::{HttpResponse, Responder, get, web};
use entity::prelude::Applications;
use sea_orm::EntityTrait;

#[get("/applications")]
pub async fn get(state: web::Data<State>) -> impl Responder {
    let applications = Applications::find()
        .into_json()
        .all(&state.db)
        .await
        .unwrap(); // todo: handle error

    HttpResponse::Ok().json(applications)
}
