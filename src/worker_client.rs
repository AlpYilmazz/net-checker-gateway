use commons::{
    message::{
        topic, CreateMonitorMessage, KillMonitorMessage, MonitorMessage, PauseMonitorMessage,
        ResumeMonitorMessage,
    },
    MonitorConfiguration, MonitorTimingSettings,
};
use pulsar::{
    proto::{schema::Type, Schema},
    ProducerOptions, Pulsar, SerializeMessage, TokioExecutor,
};

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

    async fn send_message<T: SerializeMessage>(&self, topic: &str, msg: T) {
        dbg!("Topic: {topic}, message: {message}");

        let mut producer = self
            .pulsar
            .producer()
            .with_name("producer-gateway")
            .with_topic(topic)
            .with_options(ProducerOptions {
                schema: Some(Schema {
                    r#type: Type::String as i32,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build()
            .await
            .unwrap(); // TODO

        let _result = producer.send(msg).await; // TODO
    }

    pub async fn create_new_monitor(
        &self,
        worker_id: &str,
        monitor_id: &str,
        timing: MonitorTimingSettings,
        monitor: MonitorConfiguration,
    ) {
        let create_message = MonitorMessage::Create(CreateMonitorMessage {
            monitor_id: monitor_id.to_string(),
            timing,
            monitor,
        });
        self.send_message(&topic("monitor", worker_id), create_message)
            .await
    }

    pub async fn pause_monitor(&self, worker_id: &str, monitor_id: &str) {
        let pause_message = MonitorMessage::Pause(PauseMonitorMessage {
            monitor_id: monitor_id.to_string(),
        });
        self.send_message(&topic("monitor", worker_id), pause_message)
            .await
    }

    pub async fn resume_monitor(&self, worker_id: &str, monitor_id: &str) {
        let resume_message = MonitorMessage::Resume(ResumeMonitorMessage {
            monitor_id: monitor_id.to_string(),
        });
        self.send_message(&topic("monitor", worker_id), resume_message)
            .await
    }

    pub async fn kill_monitor(&self, worker_id: &str, monitor_id: &str) {
        let kill_message = MonitorMessage::Kill(KillMonitorMessage {
            monitor_id: monitor_id.to_string(),
        });
        self.send_message(&topic("monitor", worker_id), kill_message)
            .await
    }
}
