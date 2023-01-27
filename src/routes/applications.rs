use crate::types::Application;
use actix_web::{get, web, HttpResponse, Responder};
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;

#[get("/applications")]
pub async fn get(client: web::Data<Surreal<WsClient>>) -> impl Responder {
    let applications: Vec<Application> = client.select("applications").await.unwrap();
    HttpResponse::Ok().json(applications)
}
