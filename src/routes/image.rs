use crate::types::Server;
use actix_web::{get, post, web, HttpResponse, Responder};
use hosting_types::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Deserialize, Serialize)]
pub struct Form {
    name: String,
    version: Option<String>,
}

#[post("/image/{id}")]
pub async fn download(
    client: web::Data<Surreal<Client>>,
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

#[get("/image/{id}/version/{name}")]
pub async fn version(
    client: web::Data<Surreal<Client>>,
    param: web::Path<(String, String)>,
) -> impl Responder {
    let server: Option<Server> = client.select(("servers", param.0.clone())).await.unwrap();
    if let Some(server) = server {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "http://{}:{}/image/{}/version",
                server.ip, server.port, param.1
            ))
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
            StatusCode::OK => HttpResponse::Ok().json(res.json::<Vec<String>>().await.unwrap()),
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
