use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc::*;
use tokio::task;

pub struct InputManager {
    pub task: Option<task::JoinHandle<()>>,
    pub event_sender: Sender<KeyCode>,
    pub event_receiver: Receiver<KeyCode>,
}

impl Default for InputManager {
    fn default() -> Self {
        let (event_sender, event_receiver) = channel::<KeyCode>(2);

        InputManager {
            task: Option::None,
            event_sender,
            event_receiver,
        }
    }
}

impl InputManager {
    pub fn subscribe(&mut self) {
        self.unsubscribe();

        let sender = self.event_sender.clone();
        let thread = task::spawn(async move {
            loop {
                let event = match event::read() {
                    Ok(Event::Key(e)) => e,
                    _ => continue,
                };

                let _ = sender.send(event.code).await;
            }
        });

        self.task = Some(thread);
    }

    pub fn unsubscribe(&mut self) {
        match &self.task {
            Some(t) => t.abort(),
            _ => {}
        };

        self.task = None;
    }
}

impl Drop for InputManager {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}
