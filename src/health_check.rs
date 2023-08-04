use actix_web::{delete, post, put, web, HttpResponse};
use commons::{
    db::{HealthMonitorConfiguration, MonitorType},
    HealthMonitorRequest,
};

use crate::{assign::assign_workers, service, Environment};

#[post("/{client_id}/health-monitor")]
pub async fn create_health_monitor(
    env: web::Data<Environment>,
    mongo_client: web::Data<mongodb::Client>,
    client_id: web::Path<String>,
    request: web::Json<HealthMonitorRequest>,
) -> HttpResponse {
    let request = request.0;
    let client_id = client_id.into_inner();

    let configuration =
        HealthMonitorConfiguration::as_insert_doc(&client_id, MonitorType::from(&request.monitor));

    // Save Request to DB
    let _result = mongo_client
        .database(&env.mongo_db_name)
        .collection("healthCheckConfiguration")
        .insert_one(configuration, None)
        .await;

    // Assign to Workers and Message
    let workers = service::fetch_workers(&mongo_client, &env.mongo_db_name).await;
    let worker_ids = assign_workers(&workers, &request.geo_settings);

    dbg!(&worker_ids);

    HttpResponse::Created().body("OK")
}

#[put("/{client_id}/health-monitor/{monitor_id}")]
pub async fn start_stop_health_monitor(
    env: web::Data<Environment>,
    mongo_client: web::Data<mongodb::Client>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    // Update DB

    // Send Start/Stop Message to Worker

    HttpResponse::Ok().body("DELETED")
}

#[delete("/{client_id}/health-monitor/{monitor_id}")]
pub async fn delete_health_monitor(
    env: web::Data<Environment>,
    mongo_client: web::Data<mongodb::Client>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    // Delete from DB

    // Send Delete Message to Worker

    HttpResponse::Ok().body("DELETED")
}
