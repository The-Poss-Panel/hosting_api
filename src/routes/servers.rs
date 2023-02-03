use crate::types::Server;
use actix_web::{get, web, HttpResponse, Responder};
use surrealdb::{Surreal, engine::remote::ws::Client};

#[get("/servers")]
pub async fn get(client: web::Data<Surreal<Client>>) -> impl Responder {
    let servers: Vec<Server> = client.select("servers").await.unwrap();
    HttpResponse::Ok().json(servers)
}
