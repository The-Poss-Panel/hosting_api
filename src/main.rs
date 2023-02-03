mod routes;
use actix_cors::Cors;
use env_logger::Builder;
use log::LevelFilter;
use routes::{application, applications, image, images, server, servers};

mod types;

use actix_web::{web, App, HttpServer};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use types::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let client = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();

    client.use_ns("test").use_db("test").await.unwrap();

    client.delete("applications").await.unwrap();
    client.delete("servers").await.unwrap();
    let _server: Server = client
        .create("servers")
        .content(Server::new(
            "127.0.0.1".into(),
            8082,
            "test".into(),
            "MoskalykA".into(),
        ))
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(client.clone()))
            .service(application::find)
            .service(application::create)
            .service(application::actions)
            .service(applications::get)
            .service(server::find)
            .service(server::modify)
            .service(servers::get)
            .service(image::download)
            .service(image::version)
            .service(images::get)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
