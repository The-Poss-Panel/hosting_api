use crate::types::Application;
use actix_web::{get, web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[get("/applications")]
pub async fn get(client: web::Data<Surreal<Client>>) -> impl Responder {
    let applications: Vec<Application> = client.select("applications").await.unwrap();
    HttpResponse::Ok().json(applications)
}
