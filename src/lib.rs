use std::{collections::HashSet, usize};

use raylib::prelude::*;
use array2d::Array2D;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    color: Color, 
    rhythm: Option<TileRhythm>
    // todo: add more features
}

impl Tile {
    pub fn from(color: &Color, rhythm: Option<TileRhythm> ) -> Self{
        Tile {
            color: *color,
            rhythm,
        }
    }
    pub fn update(&mut self, delta: Sec){
        self.rhythm.as_mut().map(|r|r.update(delta));
    }

    pub fn get_color(&self) -> Color {
        match &self.rhythm {
            None => self.color,
            Some(tile_rhythm) => {
                if tile_rhythm.on() {
                    self.color
                } else {
                    Color::new(0, 0, 0, 0)
                }
            }
        }
    }
}

pub type Sec = f64;
pub type BPM = f64;

pub fn beat_length(tempo: BPM) -> Sec {
    60_f64 / tempo as Sec
}


#[derive(Debug,Default,Clone)]
pub struct TileRhythm {
    // Number of beats in a measure
    length: usize,
    // Length of a beat, in seconds
    duration: Sec,
    // which beats to play; zero-indexed
    beats: HashSet<usize>,
    // 
    time: Sec,
}

impl TileRhythm{
    pub fn update(&mut self, delta: Sec){
        self.time += delta;
        self.time = self.time % (self.duration * self.length as Sec);
    }

    pub fn on(&self) -> bool {
        self.beats.contains(& ((self.time / self.duration) as usize))
    }
    pub fn new<T>(length: usize, tempo: BPM, beats: T) -> Self 
    where T: IntoIterator<Item=usize> {
        TileRhythm{
            length, 
            duration: beat_length(tempo),
            beats: beats.into_iter().collect(),
            time: 0.0
        }
    }
}

#[test]
fn test_rhythm(){
    let mut r = TileRhythm::new(2, 120.0, vec![1]);
    assert_eq!(r.on(), false);
    r.update(0.15);
    assert_eq!(r.on(), false);
    assert!((r.time - 0.15).abs() < 1e-6);
    r.update(r.duration);
    assert!(r.on());
    r.update(r.duration);
    assert_eq!(r.on(), false);
    assert!((r.time - 0.15).abs() < 1e-6);
}


pub struct Level {
    pub tiles: Array2D<Tile>,
    tile_width: i32,
    tile_height: i32,
    row_gap: i32,
    column_gap: i32,
    // todo: add more features
}

impl Level {
    pub fn new(tiles: Array2D<Tile>, tile_width: i32, tile_height: i32, row_gap: i32, column_gap: i32) -> Self{
        Level {
            tiles,
            tile_width,
            tile_height,
            row_gap,
            column_gap
        }
    }
    pub fn draw(&self, handle: &mut RaylibDrawHandle){
        for ((row,col), tile) in self.tiles.enumerate_column_major() {
            handle.draw_rectangle(row as i32 * self.tile_width + self.row_gap / 2,
                col as i32 * self.tile_height + self.column_gap / 2,
                self.tile_width - self.row_gap / 2,
                self.tile_height - self.column_gap / 2,
                tile.get_color()
            );
        }
    }

    pub fn update(&mut self, delta: Sec){
        for (r,c) in self.tiles.indices_column_major(){
            let tile = self.tiles.get_mut(r, c).unwrap();
            tile.update(delta)
        }
    }
}