use crate::State;
use actix_web::{HttpResponse, Responder, get, post, web};
use bollard::query_parameters::CreateImageOptionsBuilder;
use futures_util::TryStreamExt;
use hosting_types::Response;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Form {
    name: String,
    version: Option<String>,
}

#[post("/image/{id}")]
pub async fn download(
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

    if server.inspect_image(&form.name).await.is_ok() {
        return HttpResponse::Found().json(Response {
            error: true,
            message: "The image has already been downloaded".into(),
        });
    };

    let options = CreateImageOptionsBuilder::new()
        .from_image(&form.name)
        .tag(&form.version.clone().unwrap_or_else(|| "latest".to_string())); // todo: maybe just str

    match server
        .create_image(Some(options.build()), None, None)
        .try_collect::<Vec<_>>()
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            error: false,
            message: "The image to be downloaded".into(),
        }),
        Err(_) => HttpResponse::NotFound().json(Response {
            error: true,
            message: "There is an error when downloading the image".into(),
        }),
    }
}

#[get("/image/{id}/version/{name}")]
pub async fn version(state: web::Data<State>, path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
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

    match server.inspect_image(&name).await {
        Ok(images) => HttpResponse::Ok().json(images.repo_tags.unwrap_or_default()),
        Err(_) => HttpResponse::NotFound().json(Response {
            error: false,
            message: "Image recovery does not work".into(),
        }),
    }
}
