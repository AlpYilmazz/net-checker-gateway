use commons::{
    message::{
        topic, CreateMonitorMessage, KillMonitorMessage, PauseMonitorMessage, ResumeMonitorMessage,
    },
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
        Self { pulsar }
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
            .build()
            .await
            .unwrap(); // TODO

        let _result = producer.send(message).await; // TODO
    }

    pub async fn create_new_monitor(
        &self,
        worker_id: &str,
        monitor_id: &str,
        timing: MonitorTimingSettings,
        monitor: MonitorConfiguration,
    ) {
        let create_message = CreateMonitorMessage {
            monitor_id: monitor_id.to_string(),
            timing,
            monitor,
        };
        let message = serde_json::to_string(&create_message).unwrap();
        self.send_message(&topic("create", worker_id), &message)
            .await
    }

    pub async fn pause_monitor(&self, worker_id: &str, monitor_id: &str) {
        let pause_message = PauseMonitorMessage {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&pause_message).unwrap();
        self.send_message(&topic("pause", worker_id), &message)
            .await
    }

    pub async fn resume_monitor(&self, worker_id: &str, monitor_id: &str) {
        let resume_message = ResumeMonitorMessage {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&resume_message).unwrap();
        self.send_message(&topic("resume", worker_id), &message)
            .await
    }

    pub async fn kill_monitor(&self, worker_id: &str, monitor_id: &str) {
        let delete_message = KillMonitorMessage {
            monitor_id: monitor_id.to_string(),
        };
        let message = serde_json::to_string(&delete_message).unwrap();
        self.send_message(&topic("kill", worker_id), &message).await
    }
}
