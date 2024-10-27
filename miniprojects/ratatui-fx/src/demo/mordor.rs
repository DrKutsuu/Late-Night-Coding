use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use rand_seeder::Seeder;
use ratatui::style::Color::{Black, Blue, Cyan, DarkGray, Gray, Green, LightBlue, LightCyan, LightGreen, LightMagenta, LightRed, LightYellow, Magenta, Red, White, Yellow};
use crate::ui::framehandler::{FrameHandler};

const PATTERN_A: &str = "....>";
const PATTERN_B: &str = "^ ....";
const PATTERN_C: &str = "0....";
const PATTERN_D: &str = "1....";
const PATTERN_E: &str = "X";
const PATTERN_F: &str = "/";

pub struct MordorData {
    pub position: Position,
    pub text_color: Color
}

pub struct MordorFrameHandler {
    pub color_map : HashMap<i32, Color>,
    pub rng : StdRng
}

/*
  Mapping to replicate the FreePascal CRT color constants
  https://wiki.freepascal.org/Crt#Color_Constants:
  
  Black        =   0;
  Blue         =   1;
  Green        =   2;
  Cyan         =   3;
  Red          =   4;
  Magenta      =   5;
  Brown        =   6;
  LightGray    =   7;
  DarkGray     =   8;
  LightBlue    =   9;
  LightGreen   =  10;
  LightCyan    =  11;
  LightRed     =  12;
  LightMagenta =  13;
  Yellow       =  14;
  White        =  15;
  Blink        = 128;
 */
fn build_colour_map() -> HashMap<i32, Color> {
    let mut map = HashMap::new();
    map.insert(0, Black);
    map.insert(1, Blue);
    map.insert(2, Green);
    map.insert(3, Cyan);
    map.insert(4, Red);
    map.insert(5, Magenta);
    map.insert(6, LightYellow); // Best match for "Brown"?
    map.insert(7, Gray); // LightGray
    map.insert(8, DarkGray);
    map.insert(9, LightBlue);
    map.insert(10, LightGreen);
    map.insert(11, LightCyan);
    map.insert(12, LightRed);
    map.insert(13, LightMagenta);
    map.insert(14, Yellow);
    map.insert(15, White);
    map
}

fn build_paragraph(text: &str, color: Color) -> Paragraph {
    Paragraph::new(text)
        .style(Style::default()
            .fg(color))
}

impl FrameHandler<MordorData> for MordorFrameHandler {
    fn handle_frame(&mut self, frame: &mut Frame, data: &mut MordorData) {
        for i in 0..50
        {
            let position = data.position;
            let color = data.text_color;
            frame.render_widget(
                build_paragraph(PATTERN_A, color.clone()),
                Rect::new(position.x, position.y, PATTERN_A.len() as u16, 1)
            );

            frame.render_widget(
                build_paragraph(PATTERN_B, color.clone()),
                Rect::new(position.x + 1, position.y + 1, PATTERN_B.len() as u16, 1)
            );

            frame.render_widget(
                build_paragraph(PATTERN_C, color.clone()),
                Rect::new(position.x + 2, position.y + 2, PATTERN_C.len() as u16, 1)
            );

            frame.render_widget(
                build_paragraph(PATTERN_D, color.clone()),
                Rect::new(position.x + 3, position.y + 3, PATTERN_D.len() as u16, 1)
            );

            frame.render_widget(
                build_paragraph(PATTERN_E, color.clone()),
                Rect::new(position.x + 4, position.y + 4, PATTERN_E.len() as u16, 1)
            );

            frame.render_widget(
                build_paragraph(PATTERN_F, color.clone()),
                Rect::new(position.x + 5, position.y + 5, PATTERN_F.len() as u16, 1)
            );

            // Finally, re-randomise the position and color data
            let new_x = self.rng.gen_range(0..frame.area().width - 10);
            let new_y = self.rng.gen_range(0..frame.area().height - 5);
            data.position = Position::new(new_x, new_y);
            let new_color_choice = self.rng.gen_range(0..=15);
            let new_color = self.color_map.get(&new_color_choice).unwrap();
            data.text_color = *new_color
        }
    }
}

impl MordorData {
    pub fn new() -> MordorData {
        let position = Position::new(0,0);
        let text_color = Color::White;
        
        MordorData {
            position,
            text_color
        }
    }
}

impl  MordorFrameHandler {
    pub fn new(seed_input: String) -> MordorFrameHandler {
        let seed = Seeder::from(seed_input).make_seed();
        let rng = StdRng::from_seed(seed);
        MordorFrameHandler {
            color_map: build_colour_map(),
            rng,
        }
    }
}