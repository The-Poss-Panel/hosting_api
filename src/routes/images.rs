use crate::types::Server;
use actix_web::{get, web, HttpResponse, Responder};
use bollard::service::ImageSummary;
use hosting_types::Response;
use reqwest::StatusCode;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[get("/images/{id}")]
pub async fn get(client: web::Data<Surreal<Client>>, id: web::Path<String>) -> impl Responder {
    let server: Option<Server> = client.select(("servers", id.to_string())).await.unwrap();
    if let Some(server) = server {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("http://{}:{}/images", server.ip, server.port))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::NOT_FOUND => {
                HttpResponse::NotFound().json(res.json::<Response>().await.unwrap())
            }
            StatusCode::OK => {
                HttpResponse::Ok().json(res.json::<Vec<ImageSummary>>().await.unwrap())
            }
            _ => HttpResponse::NotFound().json(Response {
                error: true,
                message: "This protocol does not exist".into(),
            }),
        }
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server does not exist".into(),
        })
    }
}
