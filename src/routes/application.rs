use crate::types::{Application, Server};
use actix_web::{get, post, web, HttpResponse, Responder};
use bollard::service::PortBinding;
use hosting_types::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Form {
    name: String,
    image: String,
    ports: Option<Vec<PortBinding>>,
}

#[derive(Serialize, Deserialize)]
pub struct Actions {
    action: String,
}

#[get("/application/{id}")]
pub async fn find(client: web::Data<Surreal<WsClient>>, id: web::Path<String>) -> impl Responder {
    let application: Option<Application> = client
        .select(("applications", id.to_string()))
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
    surreal: web::Data<Surreal<WsClient>>,
    id: web::Path<String>,
    form: web::Json<Form>,
) -> impl Responder {
    let server: Option<Server> = surreal.select(("servers", id.clone())).await.unwrap();
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
                let _application: Application = surreal
                    .create("applications")
                    .content(Application {
                        id: Ulid::new().to_string(),
                        name: form.name.clone(),
                        image: form.image.clone(),
                        owner: "MoskalykA".into(),
                        server: id.clone(),
                        ports: Some(form.ports.clone().unwrap()),
                    })
                    .await
                    .unwrap();

                HttpResponse::Ok().json(res.json::<Response>().await.unwrap())
            }
            _ => HttpResponse::Ok().json(res.json::<Response>().await.unwrap()),
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
    client: web::Data<Surreal<WsClient>>,
    id: web::Path<String>,
    form: web::Json<Actions>,
) -> impl Responder {
    let application: Option<Application> =
        client.select(("applications", id.clone())).await.unwrap();
    if let Some(application) = application {
        let server: Option<Server> = client
            .select(("servers", application.server.clone()))
            .await
            .unwrap();
        if let Some(server) = server {
            let client = reqwest::Client::new();
            let res = client
                .post(format!(
                    "http://{}:{}/application/{}/actions",
                    server.ip, server.port, application.name
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
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: format!("The {} server does not exist", application.server),
            });
        }
    } else {
        return HttpResponse::NotFound().json(Response {
            error: true,
            message: format!("The {} application does not exist", id),
        });
    }
}
