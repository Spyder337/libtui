use std::{cell::RefCell, error};

use crossterm::event::{KeyEvent as CrosstermKeyEvent, MouseEvent};
use ratatui::{backend::Backend, Terminal};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

pub mod app;
pub mod events;
pub mod program;
pub mod tui;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;
pub type CrosstermBackend = ratatui::backend::CrosstermBackend<std::io::Stderr>;
pub type Term = Tui<CrosstermBackend>;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(CrosstermKeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

pub trait Renderer {
    fn render(&self, frame: &mut ratatui::prelude::Frame<'_>);
}

pub trait KeyEventHandler {
    fn handle_key_event(&mut self, ke: CrosstermKeyEvent) -> AppResult<()>;
}

pub trait Executable {
    fn is_running(&self) -> bool;
    fn quit(&mut self);
    fn tick(&self) {}
    fn can_print(&self) -> bool {
        false
    }
}

/// Default counter application example.
/// Just plug it into a [`Program`] and run it.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub do_print: bool,
}

/**
Default example of a program using the [`ratatui`] library.
It contains an [`App`] struct that contains the data for the application
and also implements the [`Renderer`], [`KeyEventHandler`], [`Executable`] traits.
*/
pub struct Program<T: Renderer + KeyEventHandler + Executable + Serialize> {
    /// Represents the program state.
    /// T also has ties to functions that allow a program to run.
    ///
    app: RefCell<T>,
}

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}
