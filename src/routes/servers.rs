use crate::types::Server;
use actix_web::{get, web, HttpResponse, Responder};
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;

#[get("/servers")]
pub async fn get(client: web::Data<Surreal<WsClient>>) -> impl Responder {
    let servers: Vec<Server> = client.select("servers").await.unwrap();
    HttpResponse::Ok().json(servers)
}
