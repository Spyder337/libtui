use crossterm::event::{KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use ratatui::{layout::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Paragraph}
};

use crate::{App, CrosstermBackend, Event, Tui};

use super::{AppResult, Executable, KeyEventHandler, Renderer};

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            do_print: true,
            counter: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

impl Renderer for App {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame<'_>) {
        frame.render_widget(
            Paragraph::new(format!(
                "This is a tui template.\n\
                    Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                    Press left and right to increment and decrement the counter respectively.\n\
                    Counter: {}",
                self.counter
            ))
            .block(
                Block::bordered()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
            frame.size(),
        )
    }
}

impl KeyEventHandler for App {
    fn handle_key_event(&mut self, ke: CrosstermKeyEvent) -> AppResult<()> {
        match ke.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if ke.modifiers == KeyModifiers::CONTROL {
                    self.quit();
                }
            }
            // Counter handlers
            KeyCode::Right => {
                self.increment_counter();
            }
            KeyCode::Left => {
                self.decrement_counter();
            }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }
}

impl Executable for App {
    /// Handles the tick event of the terminal.
    fn tick(&self) {}

    /// Is the app running?
    fn is_running(&self) -> bool {
        self.running
    }

    async fn run(&mut self, tui: &mut Tui<CrosstermBackend>) -> AppResult<bool>{
        while self.is_running() {
            tui.draw(self)?;

            match tui.events.next().await? {
                Event::Tick => self.tick(),
                Event::Key(ke) => self.handle_key_event(ke)?,
                Event::Mouse(_) => (),
                Event::Resize(_, _) => (),
            }
        }
        Ok(self.do_print)
    }
    
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
