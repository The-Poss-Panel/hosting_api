use crate::types::Server;
use actix_web::{post, web, HttpResponse, Responder};
use hosting_types::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;

#[derive(Deserialize, Serialize)]
pub struct Form {
    name: String,
    version: Option<String>,
}

#[post("/image/{id}")]
pub async fn download(
    client: web::Data<Surreal<WsClient>>,
    id: web::Path<String>,
    form: web::Json<Form>,
) -> impl Responder {
    let server: Option<Server> = client.select(("servers", id.to_string())).await.unwrap();
    if let Some(server) = server {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("http://{}:{}/image", server.ip, server.port))
            .json::<Form>(&form)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::FOUND => {
                HttpResponse::NotFound().json(res.json::<Response>().await.unwrap())
            }
            StatusCode::NOT_FOUND => {
                HttpResponse::NotFound().json(res.json::<Response>().await.unwrap())
            }
            StatusCode::OK => HttpResponse::Ok().json(res.json::<Response>().await.unwrap()),
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
