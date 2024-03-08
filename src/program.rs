use std::io::stderr;


use crate::{ AppResult, CrosstermBackend, Event, Executable, KeyEventHandler, Renderer, Term };
use crate::Program;


use ratatui::Terminal;
use serde::Serialize;
use serde_json::json;

impl<T: Renderer + KeyEventHandler + Executable + Serialize + Copy> Program<T> {
    /**
    Constructs a new instance of [`Program`].
    */
    pub fn new(app: T) -> Self {
        Self { app }
    }

    pub async fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(stderr());
        let terminal = Terminal::new(backend)?;
        let events = crate::EventHandler::new(250);
        let mut tui = Term::new(terminal, events);

        tui.init()?;

        
        loop {
            if self.app.is_running() {
                break;
            }

            tui.draw(self.app)?;

            match tui.events.next().await? {
                Event::Tick => self.app.tick(),
                Event::Key(ke) => self.app.handle_key_event(ke)?,
                Event::Mouse(_) => (),
                Event::Resize(_, _) => (),
            }
        }

        tui.exit()?;

        if self.app.can_print() {
            println!("{}", json!(self.app));
        } 

        Ok(())
    }
}
