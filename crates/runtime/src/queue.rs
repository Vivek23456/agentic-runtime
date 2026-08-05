use tokio::sync::mpsc;

use crate::task::Task;

pub type TaskSender = mpsc::Sender<Task>;
pub type TaskReceiver = mpsc::Receiver<Task>;

pub fn task_channel(capacity: usize) -> (TaskSender, TaskReceiver) {
    mpsc::channel(capacity)
}