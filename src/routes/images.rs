use crate::State;
use actix_web::{get, web, HttpResponse, Responder};
use bollard::service::ImageSummary;
use entity::prelude::Servers;
use hosting_types::Response;
use reqwest::StatusCode;
use sea_orm::EntityTrait;

#[get("/images/{id}")]
pub async fn get(state: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let server = Servers::find_by_id(id).one(&state.db).await.unwrap();

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
