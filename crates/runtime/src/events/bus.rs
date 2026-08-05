use tokio::sync::broadcast;

use super::RuntimeEvent;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<RuntimeEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);

        Self { sender }
    }

    pub fn publish(
        &self,
        event: RuntimeEvent,
    ) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(
        &self,
    ) -> broadcast::Receiver<RuntimeEvent> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn publishes_events() {
        let bus = EventBus::new(16);

        let mut receiver = bus.subscribe();

        let id = Uuid::new_v4();

        bus.publish(RuntimeEvent::TaskCreated {
            task_id: id,
        });

        let event = receiver.recv().await.unwrap();

        match event {
            RuntimeEvent::TaskCreated { task_id } => {
                assert_eq!(task_id, id);
            }
            _ => panic!("unexpected event"),
        }
    }
}