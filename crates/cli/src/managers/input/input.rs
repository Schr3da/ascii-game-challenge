use crossterm::event::{Event, KeyEvent, MouseEvent};
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
            loop {
                match crossterm::event::read() {
                    Ok(Event::Key(e)) => {
                        _ = sender.send(InpuEvents::Keyboard(e)).await;
                    }
                    Ok(Event::Mouse(e)) => {
                        _ = sender.send(InpuEvents::Mouse(e)).await;
                    }
                    Ok(Event::Resize(columns, rows)) => {
                        _ = sender.send(InpuEvents::Window(WindowEvents::Resize(columns, rows))).await;
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
