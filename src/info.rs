use actix_web::{get, web, HttpResponse};
use mongodb::bson::doc;
use serde::Serialize;

use crate::{service, Environment};

#[derive(Serialize)]
pub struct WorkerResponse {
    id: String,
    name: String,
    city: String,
    country: String,
    region: String,
}

#[get("/locations")]
pub async fn get_workers(
    env: web::Data<Environment>,
    mongo_client: web::Data<mongodb::Client>,
) -> HttpResponse {
    let workers = service::fetch_workers(&mongo_client, &env.mongo_db_name).await;

    let response: Vec<WorkerResponse> = workers
        .into_iter()
        .map(|location| WorkerResponse {
            id: location._id,
            name: location.name,
            city: location.city,
            country: location.country,
            region: location.region,
        })
        .collect();

    HttpResponse::Ok().json(response)
}
