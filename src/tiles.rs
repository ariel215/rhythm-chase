use core::panic;
use iset;
use raylib::{color::Color, drawing::{RaylibDraw,RaylibDrawHandle}};
use serde::*;
use std::hash::Hash;

use crate::{TileDimensions, BPM};

pub type Sec = f64;

pub trait GameState {
    fn update(&mut self, delta: Sec);
}

pub trait GamePiece {
    fn draw(&self, canvas: &mut RaylibDrawHandle);

}
type Fraction = f64;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Measure{
    notes: iset::IntervalSet<Fraction>
}

impl Measure {
    pub fn is_on(&self, position: Fraction) -> bool {
        self.notes.has_overlap(position..=position)
    }

    pub fn add_note(&mut self, position: f64, duration: f64){
        let end = position + duration;
        if end <= 1. {
            self.notes.insert(position..end);
        } else {
            panic!()
        }
    }

    pub fn in_window(&self, position: Fraction, size: Fraction) -> bool {
        let start = position - size;
        let end  = position + size;
        if self.notes.has_overlap(start..end){
            return true;
        }

        if (end > 1. )&& self.notes.has_overlap(1.0-end..0. + f64::EPSILON) {
            return true;
        }
        if start < 0.0 && self.notes.has_overlap(1.0+start..1.0 + f64::EPSILON){
            return true;
        }
        return false;
    }
}


#[test]
fn test_measure(){
    let mut m = Measure::default();
    m.add_note(0.0, 0.5);
    assert!(m.is_on(0.25));
    assert!(m.in_window(0.05, 0.05));
    assert!(m.in_window(0.975, 0.05));
    
    let mut m = Measure::default();
    m.add_note(0.5, 0.49);
    assert!(m.in_window(0.475, 0.05))
}


// Struct representing a single repeated rhythmic pattern
#[derive(Debug,Clone,Deserialize,Serialize, PartialEq, PartialOrd)]
pub struct Rhythm {
    pub len: Sec,
    pub beats: Measure,
    #[serde(skip)]
    time: Sec
}

impl GameState for Rhythm{
    fn update(&mut self, delta: Sec) {
        self.time += delta;
        self.time %= self.len
    }
}

impl Rhythm{

    pub fn on(&self) -> bool {
        self.beats.is_on(self.position())
    }

    pub fn position(&self) -> f64 {
        self.time / self.len
    }

    pub fn in_window(&self, window_size: Sec)-> bool {
        self.beats.in_window(self.position(), window_size)
    }

    pub fn new(length: Sec, beats: Measure) -> Self {
        Self {
            len: length,
            beats,
            time:0.0
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Tile {
    rhythm: Rhythm,
    position: (usize,usize),
    color: Color,
    #[serde(skip)]
    dimensions: Option<std::rc::Rc<TileDimensions>>
}

impl GameState for Tile {
    fn update(&mut self, delta: Sec) {
        self.rhythm.update(delta)
    }
}

impl GamePiece for Tile {
    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        if let Some(dims) = &self.dimensions {
            let (row,col) = self.position;
            let (x_tl,y_tl) = dims.top_left(row as i32, col as i32);
            let (x_br, y_br) = dims.bottom_right(row as i32, col as i32);
            canvas.draw_rectangle(x_tl,y_tl,
                x_br - x_tl,
                y_br - y_tl,
                self.color
            );

        }
    }
}
