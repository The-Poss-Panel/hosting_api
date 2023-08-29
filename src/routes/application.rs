use crate::State;
use actix_web::{get, post, web, HttpResponse, Responder};
use bollard::{
    container::{Config, CreateContainerOptions, InspectContainerOptions, StartContainerOptions},
    service::{HostConfig, PortBinding},
};
use entity::{applications, prelude::Applications};
use hosting_types::Response;
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub async fn find(state: web::Data<State>, path: web::Path<String>) -> impl Responder {
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
    path: web::Path<u32>,
    form: web::Json<Form>,
) -> impl Responder {
    let id = path.into_inner();
    let servers = state.servers.lock().await;
    let server = match servers.get(&id) {
        Some(s) => s,
        None => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: "The server does not exist".into(),
            });
        }
    };

    let split: Vec<&str> = form.image.split(':').collect::<Vec<&str>>();
    let name = split.first().unwrap();
    match server.inspect_image(name).await {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: "The image has not been downloaded".into(),
            })
        }
    };

    let port_bindings = if let Some(ports) = &form.ports {
        let mut port_bindings = HashMap::new();
        port_bindings.insert(String::from("80/tcp"), Some(ports.to_vec()));

        Some(port_bindings)
    } else {
        None
    };

    match server
        .create_container(
            Some(CreateContainerOptions {
                name: form.alias.clone(),
                platform: None,
            }),
            Config::<String> {
                image: Some(form.image.clone()),
                host_config: Some(HostConfig {
                    port_bindings,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
    {
        Ok(c) => {
            //info!("The {} container has just been created", c.id);

            let _ = Applications::insert(applications::ActiveModel {
                id: Set(c.id.clone()),
                image: Set(form.image.clone()),
                alias: Set(if form.alias.clone().is_empty() {
                    "default".to_string()
                } else {
                    form.alias.clone()
                }),
                owner: Set("MoskalykA".into()),
                server: Set(id.try_into().unwrap()), //ports: Some(form.ports.clone().unwrap())
            })
            .exec(&state.db)
            .await
            .unwrap();

            //HttpResponse::Ok().json(Response {
            //    error: false,
            //    message: format!("The application {} has been created", application.last_insert_id),
            //});

            match server.start_container::<String>(&c.id, None).await {
                Ok(_) => {
                    //info!("The container {} has just undergone a start", c.id);

                    HttpResponse::Ok().json(c.id)
                }
                Err(e) => HttpResponse::NotFound().json(Response {
                    error: true,
                    message: e.to_string(),
                }),
            }
        }
        Err(_) => HttpResponse::NotFound().json(Response {
            error: true,
            message: "There is an error when creating the application".into(),
        }),
    }
}

#[get("/application/{server_id}/state/{id}")]
pub async fn _state(
    state: web::Data<State>,
    path: web::Path<(u32, String)>
) -> impl Responder {
    let (server_id, id) = path.into_inner();
    let servers = state.servers.lock().await;
    let server = match servers.get(&server_id) {
        Some(s) => s,
        None => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: "The server does not exist".into(),
            });
        }
    };

    match server.inspect_container(&id, None).await {
        Ok(i) => HttpResponse::Ok().json(i.state),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/application/{server_id}/actions/{id}")]
pub async fn actions(
    state: web::Data<State>,
    path: web::Path<(u32, String)>,
    form: web::Json<Actions>,
) -> impl Responder {
    let (server_id, id) = path.into_inner();
    let servers = state.servers.lock().await;
    let server = match servers.get(&server_id) {
        Some(s) => s,
        None => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: "The server does not exist".into(),
            });
        }
    };

    match server
        .inspect_container(&id, Some(InspectContainerOptions { size: false }))
        .await
    {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: format!("The {} container does not exist", id),
            })
        }
    };

    match form.action.as_str() {
        "start" => match server
            .start_container(&id, None::<StartContainerOptions<String>>)
            .await
        {
            Ok(_) => {}
            Err(e) => return HttpResponse::NotFound().body(e.to_string()),
        },
        "restart" => match server.restart_container(&id, None).await {
            Ok(_) => {}
            Err(e) => return HttpResponse::NotFound().body(e.to_string()),
        },
        "stop" => match server.stop_container(&id, None).await {
            Ok(_) => {}
            Err(e) => return HttpResponse::NotFound().body(e.to_string()),
        },
        _ => {
            return HttpResponse::NotFound().json(Response {
                error: true,
                message: format!("The operation {} does not exist", form.action),
            })
        }
    };

    //info!(
    //    "The container {} has just undergone a {}",
    //    id, form.action
    //);

    HttpResponse::Ok().json(Response {
        error: false,
        message: format!(
            "The operation that allowed the {} of the {} container worked",
            form.action, id
        ),
    })
}
