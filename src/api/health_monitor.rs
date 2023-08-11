use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use commons::{
    db::{HealthMonitor, MonitorType},
    HealthMonitorRequest, StatusChangeRequest,
};
use mongodb::bson::{doc, Document};

use crate::{assign::assign_workers, service, worker_client::WorkerClient, Environment};

#[post("/{client_id}/health-monitor")]
pub async fn create_health_monitor(
    env: Data<Environment>,
    mongo_client: Data<mongodb::Client>,
    worker_client: Data<WorkerClient>,
    client_id: Path<String>,
    request: Json<HealthMonitorRequest>,
) -> HttpResponse {
    let request = request.0;
    let client_id = client_id.into_inner();

    // Assign to Workers
    // worker_ids[i] -> geo_settings.locations[i]
    let workers = service::fetch_workers(&mongo_client, &env.mongo_db_name).await;
    let worker_ids = assign_workers(
        &workers,
        request.geo_settings.locations.iter().map(|(loc, _)| loc),
    )
    .unwrap(); // TODO

    let health_monitor = HealthMonitor::as_insert_doc(
        &client_id,
        MonitorType::from(&request.monitor),
        &worker_ids,
        &request.monitor,
    );

    // Save Request to DB
    let _result = mongo_client
        .database(&env.mongo_db_name)
        .collection("healthMonitor")
        .insert_one(health_monitor, None)
        .await;

    dbg!(&worker_ids);

    // Message Workers to Create Monitors
    // TODO: await all
    for (worker_id, (_, timing)) in worker_ids
        .into_iter()
        .zip(request.geo_settings.locations.iter())
    {
        // TODO: use ref instead of clone
        worker_client
            .create_new_monitor(worker_id, timing.clone(), request.monitor.clone())
            .await;
    }

    HttpResponse::Created().body("OK")
}

#[put("/{client_id}/health-monitor/{monitor_id}")]
pub async fn start_stop_health_monitor(
    env: Data<Environment>,
    mongo_client: Data<mongodb::Client>,
    worker_client: Data<WorkerClient>,
    path: Path<(String, String)>,
    request: Json<StatusChangeRequest>,
) -> HttpResponse {
    let request = request.0;
    let (_client_id, monitor_id) = path.into_inner();

    // Update DB
    let _result = mongo_client
        .database(&env.mongo_db_name)
        .collection::<Document>("healthCheckConfiguration")
        .update_one(
            doc! { "_id": &monitor_id },
            doc! { "running": request.running },
            None,
        )
        .await;

    let health_check = mongo_client
        .database(&env.mongo_db_name)
        .collection::<HealthMonitor>("healthCheckConfiguration")
        .find_one(doc! {}, None)
        .await;

    // Send Start/Stop Message to Worker
    // match request.running {
    //     true => worker_client.start_monitor(worker_id, &monitor_id).await,
    //     false => worker_client.start_monitor(worker_id, &monitor_id).await,
    // }

    HttpResponse::Ok().body("DELETED")
}

#[delete("/{client_id}/health-monitor/{monitor_id}")]
pub async fn delete_health_monitor(
    env: Data<Environment>,
    mongo_client: Data<mongodb::Client>,
    path: Path<(String, String)>,
) -> HttpResponse {
    // Delete from DB

    // Send Delete Message to Worker

    HttpResponse::Ok().body("DELETED")
}
