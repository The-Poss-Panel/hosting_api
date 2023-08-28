use crate::State;
use actix_web::{get, post, web, HttpResponse, Responder};
use entity::prelude::Servers;
use hosting_types::Response;
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Form {
    name: String,
    version: Option<String>,
}

#[post("/image/{id}")]
pub async fn download(
    state: web::Data<State>,
    path: web::Path<i32>,
    form: web::Json<Form>,
) -> impl Responder {
    let id = path.into_inner();
    let server = Servers::find_by_id(id).one(&state.db).await.unwrap();

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
pub async fn version(state: web::Data<State>, path: web::Path<(i32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    let server = Servers::find_by_id(id).one(&state.db).await.unwrap();

    if let Some(server) = server {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "http://{}:{}/image/{}/version",
                server.ip, server.port, name
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
