use ratatui::{DefaultTerminal};
use ratatui::style::Stylize;

mod error;
mod ui;
mod demo;
mod io;

use crate::demo::mordor::{MordorData, MordorFrameHandler};
use crate::error::UnexpectedError;
use crate::io::input::check_for_escape;
use crate::ui::framehandler::FrameHandler;

fn main() -> Result<(), UnexpectedError> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = main_loop(terminal);
    ratatui::restore();
    app_result
}

fn main_loop(mut terminal: DefaultTerminal) -> Result<(), UnexpectedError> {
    let mut mordor_data = MordorData::new();
    let mordor_seed = String::from("test123");
    let mut mordor_fh: MordorFrameHandler = MordorFrameHandler::new(mordor_seed);

    loop {
        terminal.draw(|frame| {
            mordor_fh.handle_frame(frame, &mut mordor_data);
        })?;
        
        match check_for_escape() {
            Ok(pressed) => {
                if pressed { return Ok(()) }
            }, 
            _ => {}
        }
    }
}
