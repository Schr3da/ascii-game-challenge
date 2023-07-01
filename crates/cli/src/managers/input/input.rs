use crossterm::event::{Event, KeyEvent, MouseEvent};
use debounce::EventDebouncer;
use std::time::Duration;
use tokio::sync::mpsc::*;
use tokio::task;

use core_dtos::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InpuEvents {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Window(WindowEvents),
}

pub struct InputManager {
    pub task: Option<task::JoinHandle<()>>,
    pub event_sender: Sender<InpuEvents>,
    pub event_receiver: Receiver<InpuEvents>,
}

impl Default for InputManager {
    fn default() -> Self {
        let (event_sender, event_receiver) = channel::<InpuEvents>(10);

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
            let delay = Duration::from_millis(16);

            let debounced_sender = sender.clone();
            let debounce_send_event = EventDebouncer::new(delay, move |data: InpuEvents| {
                _ = debounced_sender.blocking_send(data);
            });

            loop {
                match crossterm::event::read() {
                    Ok(Event::Key(e)) => {
                        _ = sender.send(InpuEvents::Keyboard(e)).await;
                    }
                    Ok(Event::Mouse(e)) => {
                        debounce_send_event.put(InpuEvents::Mouse(e));
                    }
                    Ok(Event::Resize(columns, rows)) => {
                        debounce_send_event
                            .put(InpuEvents::Window(WindowEvents::Resize(columns, rows)));
                    }
                    _ => continue,
                };
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
