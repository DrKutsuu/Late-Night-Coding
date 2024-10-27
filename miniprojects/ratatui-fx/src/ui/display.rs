/*
 UIArea entries keyed by ID for easy reference
 */
use std::collections::HashMap;
use ratatui::layout::Position;

#[derive(Copy, Clone, std::cmp::PartialEq, Debug)]
pub struct Area {
    pub start_position : Position,
    pub end_position : Position,
    pub width: u16,
    pub height: u16
}

#[derive(Clone, Debug)]
pub struct UIArea {
    pub name : String,
    pub area: Area
}

#[derive(Clone, Debug)]
pub struct UIAreas {
    areas: HashMap<String, UIArea>
}

