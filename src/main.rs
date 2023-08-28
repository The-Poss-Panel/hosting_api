mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use bollard::Docker;
use entity::prelude::*;
use env_logger::Builder;
use log::LevelFilter;
use routes::{application, applications, image, images, server, servers};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let db: DatabaseConnection =
        Database::connect("mysql://root:YOUR_ROOT_PASSWORD_HERE@localhost:40000/poss")
            .await
            .unwrap();

    Applications::delete_many().exec(&db).await.unwrap();

    Servers::delete_many().exec(&db).await.unwrap();

    entity::servers::ActiveModel {
        ip: Set("127.0.0.1".to_string()),
        port: Set(8082),
        name: Set("test".to_string()),
        owner: Set("MoskalykA".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let mut servers: HashMap<String, Docker> = HashMap::new();
    servers.insert(
        "test".to_string(),
        Docker::connect_with_local_defaults().unwrap(),
    );

    let state = State { db };
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(servers.clone()))
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
