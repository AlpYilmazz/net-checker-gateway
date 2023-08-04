use commons::{
    message::{ChangeMonitorStateMessage, CreateMonitorMessage, DeleteMonitorMessage},
    MonitorConfiguration, MonitorTimingSettings,
};
use pulsar::{ProducerOptions, Pulsar, TokioExecutor};

#[derive(Clone)]
pub struct WorkerClient {
    pulsar: Pulsar<TokioExecutor>,
}

impl WorkerClient {
    pub async fn with_uri_str(uri: &str) -> Self {
        let pulsar = Pulsar::builder(uri, TokioExecutor)
            .build()
            .await
            .expect("Pulsar could not be created");
        Self {
            pulsar,
        }
    }

    async fn send_message(&self, topic: &str, message: &str) {
        dbg!("Topic: {topic}, message: {message}");

        let mut producer = self
            .pulsar
            .producer()
            .with_name("Gateway")
            .with_topic(topic)
            .with_options(ProducerOptions {
                // schema: todo!(),
                ..Default::default()
            })
            .build().await
            .unwrap(); // TODO

        let _result = producer.send(message).await; // TODO
    }

    fn topic(worker_id: &str) -> String {
        format!("persistent://public/default/{worker_id}")
    }

    pub async fn create_new_monitor(
        &self,
        worker_id: &str,
        timing: MonitorTimingSettings,
        monitor: MonitorConfiguration,
    ) {
        let create_message = CreateMonitorMessage { timing, monitor };
        let message = serde_json::to_string(&create_message).unwrap();
        self.send_message(&Self::topic(worker_id), &message).await
    }

    pub async fn start_monitor(&self, worker_id: &str, monitor_id: &str) {
        let change_state_message = ChangeMonitorStateMessage::Start {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&change_state_message).unwrap();
        self.send_message(&Self::topic(worker_id), &message).await
    }

    pub async fn stop_monitor(&self, worker_id: &str, monitor_id: &str) {
        let change_state_message = ChangeMonitorStateMessage::Stop {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&change_state_message).unwrap();
        self.send_message(&Self::topic(worker_id), &message).await
    }

    pub async fn delete_monitor(&self, worker_id: &str, monitor_id: &str) {
        let delete_message = DeleteMonitorMessage {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&delete_message).unwrap();
        self.send_message(&Self::topic(worker_id), &message).await
    }
}
