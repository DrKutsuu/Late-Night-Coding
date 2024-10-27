use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use crate::error::UnexpectedError;

mod error;


fn check_for_escape() -> Result<bool, UnexpectedError> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            Ok(true)
        } else {
            Ok(false)
        }
    } else { 
        Ok(false)
    }
}

fn main() -> Result<(), UnexpectedError> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = main_loop(terminal);
    ratatui::restore();
    app_result
}

fn main_loop(mut terminal: DefaultTerminal) -> Result<(), UnexpectedError> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;
        
        match check_for_escape() {
            Ok(pressed) => {
                if pressed { return Ok(()) }
            }, 
            _ => {}
        }
    }
}
