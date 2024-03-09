use std::cell::RefCell;
use std::io::stderr;

use crate::Program;
use crate::{AppResult, CrosstermBackend, Event, Executable, KeyEventHandler, Renderer, Term};

use ratatui::Terminal;
use serde::Serialize;
use serde_json::json;

impl<T: Renderer + KeyEventHandler + Executable + Serialize> Program<T> {
    /**
    Constructs a new instance of [`Program`].
    */
    pub fn new(app: T) -> Self {
        Self {
            app: RefCell::new(app),
        }
    }

    pub async fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(stderr());
        let terminal = Terminal::new(backend)?;
        let events = crate::EventHandler::new(250);
        let mut tui = Term::new(terminal, events);

        tui.init()?;

        loop {
            if self.app.borrow().is_running() {
                break;
            }

            tui.draw(self.app.borrow())?;

            match tui.events.next().await? {
                Event::Tick => self.app.borrow().tick(),
                Event::Key(ke) => self.app.borrow_mut().handle_key_event(ke)?,
                Event::Mouse(_) => (),
                Event::Resize(_, _) => (),
            }
        }

        tui.exit()?;

        if self.app.borrow().can_print() {
            println!("{}", json!(self.app));
        }

        Ok(())
    }
}
