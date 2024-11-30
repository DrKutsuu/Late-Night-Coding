use std::time::Duration;
use crossterm::event;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crate::error::UnexpectedError;

pub fn poll_event()  -> Result<Option<Event>, UnexpectedError> {
    if poll(Duration::from_millis(100))? {
        Ok(Some(event::read()?))
    } else {
        // polling timeout expired, no Event is available
        Ok(None)
    }
}

pub fn check_for_escape() -> Result<bool, UnexpectedError> {
    if let Some(Event::Key(key)) = poll_event()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}