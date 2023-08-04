use commons::db::Worker;
use futures::TryStreamExt;
use mongodb::bson::doc;

pub async fn fetch_workers(client: &mongodb::Client, db_name: &str) -> Vec<Worker> {
    let workers: Vec<Worker> = client
        .database(db_name)
        .collection::<Worker>("worker")
        .find(doc! {}, None).await
        .unwrap() // TODO
        .try_collect().await
        .unwrap(); // TODO

    workers
}