use crate::types::Server;
use actix_web::{get, post, web, HttpResponse, Responder};
use hosting_types::Response;
use serde::Deserialize;
use serde_json::json;
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;

#[derive(Deserialize)]
pub struct Form {
    name: String,
}

#[get("/server/{id}")]
pub async fn find(client: web::Data<Surreal<WsClient>>, id: web::Path<String>) -> impl Responder {
    let server: Option<Server> = client.select(("servers", id.to_string())).await.unwrap();
    if let Some(server) = server {
        HttpResponse::Ok().json(&server)
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}

#[post("/server/{id}")]
pub async fn modify(
    client: web::Data<Surreal<WsClient>>,
    id: web::Path<String>,
    form: web::Json<Form>,
) -> impl Responder {
    let server: Option<Server> = client.select(("servers", id.to_string())).await.unwrap();
    if let Some(_) = server {
        let _server: Server = client
            .update(("servers", id.to_string()))
            .merge(json!({
                "name": form.name
            }))
            .await
            .unwrap();

        HttpResponse::Ok().json(Response {
            error: false,
            message: "The server has been updated".into(),
        })
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}
