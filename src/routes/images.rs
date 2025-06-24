use crate::State;
use actix_web::{HttpResponse, Responder, get, web};
use bollard::query_parameters::ListImagesOptionsBuilder;
use hosting_types::Response;

#[get("/images/{id}")]
pub async fn get(state: web::Data<State>, path: web::Path<u32>) -> impl Responder {
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

    let options = ListImagesOptionsBuilder::new().all(true);
    match server.list_images(Some(options.build())).await {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(_) => HttpResponse::NotFound().json(Response {
            error: false,
            message: "Image recovery does not work".into(),
        }),
    }
}
