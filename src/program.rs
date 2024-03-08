
use std::io::{ stderr, Stderr };


use crate::{ AppResult, Executable, KeyEventHandler, Renderer };
use crate::{ Program, Tui };


use ratatui::Terminal;
use serde::Serialize;
use serde_json::json;

impl<T: Renderer + KeyEventHandler + Executable + Serialize> Program<T> {
    /**
    Constructs a new instance of [`Program`].
    */
    pub fn new(app: T) -> Self {
        Self { app }
    }

    pub async fn run(&mut self) -> AppResult<()> {
        type Term = Terminal<ratatui::backend::CrosstermBackend<Stderr>>;
        let backend = ratatui::backend::CrosstermBackend::new(stderr());
        let terminal = Term::new(backend)?;
        let events = crate::EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;

        let res = self.app.run(&mut tui).await;

        tui.exit()?;

        if let Ok(do_print) = res {
            if do_print {
                println!("{}", json!(self.app));
            }
        } else if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }
}
