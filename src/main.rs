use actix_web::{web, App, HttpServer};
use api::{
    health_monitor::{create_health_monitor, delete_health_monitor, start_stop_health_monitor},
    info::get_workers,
};
use worker_client::WorkerClient;

pub mod api;
pub mod assign;
pub mod service;
pub mod worker_client;

pub struct Environment {
    pub mongo_db_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_connection_string = "mongodb://localhost:27017";
    let mongo_client = mongodb::Client::with_uri_str(mongo_connection_string)
        .await
        .expect("Mongo connection failed");

    let pulsar_connection_string = "pulsar://127.0.0.1:6650";
    let worker_client = WorkerClient::with_uri_str(pulsar_connection_string).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Environment {
                mongo_db_name: "main".to_string(),
            }))
            .app_data(web::Data::new(mongo_client.clone()))
            .app_data(web::Data::new(worker_client.clone()))
            .service(get_workers)
            .service(create_health_monitor)
            .service(start_stop_health_monitor)
            .service(delete_health_monitor)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
