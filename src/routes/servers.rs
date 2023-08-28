use crate::State;
use actix_web::{get, web, HttpResponse, Responder};
use entity::prelude::Servers;
use sea_orm::EntityTrait;

#[get("/servers")]
pub async fn get(state: web::Data<State>) -> impl Responder {
    let servers = Servers::find().into_json().all(&state.db).await.unwrap(); // handle error
    HttpResponse::Ok().json(servers)
}
