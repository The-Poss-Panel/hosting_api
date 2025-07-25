mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use bollard::{API_DEFAULT_VERSION, Docker};
use entity::prelude::Servers;
use env_logger::Builder;
use futures_util::lock::Mutex;
use log::LevelFilter;
use routes::{application, applications, image, images, server, servers};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)] // todo: useful ?
pub struct State {
    db: DatabaseConnection,
    servers: Arc<Mutex<HashMap<u32, Docker>>>, // todo: maybe too slow ?
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let db: DatabaseConnection =
        Database::connect("mysql://root:YOUR_ROOT_PASSWORD_HERE@localhost:40000/poss") // todo: use .env
            .await
            .unwrap();

    let mut servers: HashMap<u32, Docker> = HashMap::new();
    let s = Servers::find().all(&db).await.unwrap();
    for (index, server) in s.iter().enumerate() {
        let a: u32 = index.try_into().unwrap();
        servers.insert(
            a + 1, // todo: wtf
            Docker::connect_with_http(
                &format!("tcp://{}:{}", &server.ip, &server.port),
                4,
                API_DEFAULT_VERSION,
            )
            .unwrap(),
        );
    }

    let state = State {
        db,
        servers: Arc::new(Mutex::new(servers)), // todo: maybe too slow ?
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            .service(application::find)
            .service(application::create)
            .service(application::_state)
            .service(application::actions)
            .service(applications::get)
            .service(server::find)
            .service(server::create)
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
