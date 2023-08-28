use crate::State;
use actix_web::{get, post, web, HttpResponse, Responder};
use bollard::service::PortBinding;
use entity::{
    applications,
    prelude::{Applications, Servers},
};
use hosting_types::Response;
use reqwest::StatusCode;
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Form {
    pub image: String,
    pub alias: String,
    pub ports: Option<Vec<PortBinding>>,
}

#[derive(Serialize, Deserialize)]
pub struct Actions {
    action: String,
}

#[get("/application/{id}")]
pub async fn find(state: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let application = Applications::find_by_id(id)
        .into_json()
        .one(&state.db)
        .await
        .unwrap();

    if let Some(application) = application {
        HttpResponse::Ok().json(&application)
    } else {
        HttpResponse::NotFound().body("The application was not found.")
    }
}

#[post("/application/{id}")]
pub async fn create(
    state: web::Data<State>,
    path: web::Path<i32>,
    form: web::Json<Form>,
) -> impl Responder {
    let id = path.into_inner();
    let server = Servers::find_by_id(id).one(&state.db).await.unwrap();

    if let Some(server) = server {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("http://{}:{}/application", server.ip, server.port))
            .json(&form)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::NOT_FOUND => {
                HttpResponse::NotFound().json(res.json::<Response>().await.unwrap())
            }
            StatusCode::OK => {
                let application_id: String = res.json().await.unwrap();
                Applications::insert(applications::ActiveModel {
                    image: Set(form.image.clone()),
                    alias: Set(if form.alias.clone().is_empty() {
                        "default".to_string()
                    } else {
                        form.alias.clone()
                    }),
                    owner: Set("MoskalykA".into()),
                    server: Set(id),
                    //ports: Some(form.ports.clone().unwrap()),
                    ..Default::default()
                });

                HttpResponse::Ok().json(Response {
                    error: false,
                    message: format!("The application {application_id} has been created"),
                })
            }
            _ => unimplemented!(),
        }
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}

#[post("/application/{id}/actions")]
pub async fn actions(
    state: web::Data<State>,
    path: web::Path<i32>,
    form: web::Json<Actions>,
) -> impl Responder {
    let id = path.into_inner();
    let application = Applications::find_by_id(id).one(&state.db).await.unwrap();

    if let Some(application) = application {
        let server = Servers::find_by_id(application.server)
            .one(&state.db)
            .await
            .unwrap();

        if let Some(server) = server {
            let client = reqwest::Client::new();
            let res = client
                .post(format!(
                    "http://{}:{}/application/{}/actions",
                    server.ip, server.port, id
                ))
                .json(&form)
                .send()
                .await
                .unwrap();

            match res.status() {
                StatusCode::NOT_FOUND => {
                    HttpResponse::NotFound().json(res.json::<Response>().await.unwrap())
                }
                _ => HttpResponse::Ok().json(res.json::<Response>().await.unwrap()),
            }
        } else {
            HttpResponse::NotFound().json(Response {
                error: true,
                message: format!("The {} server does not exist", application.server),
            })
        }
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: format!("The {id} application does not exist"),
        })
    }
}
