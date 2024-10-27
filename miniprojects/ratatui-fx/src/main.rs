use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rand::rngs::StdRng;
use rand_chacha::ChaCha12Rng;
use rand_pcg::{Lcg64Xsh32, Pcg64};
use ratatui::{DefaultTerminal, Frame};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use crate::demo::mordor::{MordorData, MordorFrameHandler};
use crate::error::UnexpectedError;
use crate::ui::framehandler::FrameHandler;

mod error;
mod ui;
mod demo;


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
    let mut mordor_data = MordorData::new();
    let mordor_seed = String::from("test123");
    let mut mordor_fh: MordorFrameHandler = MordorFrameHandler::new(mordor_seed);

    loop {
        terminal.draw(|frame| {
            mordor_fh.handle_frame(frame, &mut mordor_data);
        })?;
        
        // match check_for_escape() {
        //     Ok(pressed) => {
        //         if pressed { return Ok(()) }
        //     }, 
        //     _ => {}
        // }
    }
}
