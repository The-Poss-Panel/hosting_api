use crate::State;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use bollard::{Docker, API_DEFAULT_VERSION};
use entity::{prelude::Servers, servers};
use hosting_types::Response;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create {
    name: Option<String>,
    ip: String,
    port: u32,
}
#[derive(Deserialize)]
pub struct Modify {
    name: String,
}

#[get("/server/{id}")]
pub async fn find(state: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let server = Servers::find_by_id(id)
        .into_json()
        .one(&state.db)
        .await
        .unwrap();

    if let Some(server) = server {
        HttpResponse::Ok().json(&server)
    } else {
        HttpResponse::NotFound().json(Response {
            error: true,
            message: "The server was not found".into(),
        })
    }
}

#[post("/server")]
pub async fn create(state: web::Data<State>, form: web::Json<Create>) -> impl Responder {
    let server = match Docker::connect_with_http(
        &format!("tcp://{}:{}", &form.ip, &form.port),
        4,
        API_DEFAULT_VERSION,
    ) {
        Ok(s) => s,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    if let Some(err) = server.ping().await.err() {
        return HttpResponse::InternalServerError().body(err.to_string());
    };

    let mut servers = state.servers.lock().await;
    let len = servers.len();
    servers.insert((len + 1).try_into().unwrap(), server);

    if Servers::insert(servers::ActiveModel {
        name: Set(form.name.clone().unwrap_or("Give me a name 😍".to_string())),
        ip: Set(form.ip.clone()),
        port: Set(form.port),
        owner: Set("MoskalykA".to_string()),
        ..Default::default()
    })
    .exec(&state.db)
    .await
    .is_ok()
    {
        HttpResponse::Created().body("Your server has been successfully created")
    } else {
        HttpResponse::InternalServerError().body("There was a problem with your interaction")
    }
}

#[patch("/server/{id}")]
pub async fn modify(
    state: web::Data<State>,
    path: web::Path<i32>,
    form: web::Json<Modify>,
) -> impl Responder {
    let id = path.into_inner();

    if Servers::update(servers::ActiveModel {
        name: Set(form.name.to_string()),
        ..Default::default()
    })
    .filter(servers::Column::Id.eq(id))
    .exec(&state.db)
    .await
    .is_ok()
    {
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
