use raylib::prelude::*;
use array2d::Array2D;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub color: Color, 
    // todo: add more features
}

impl Tile {
    pub fn from(color: &Color) -> Self{
        Tile { color: *color}
    }
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
        for row in 0..self.tiles.row_len(){
            for col in 0..self.tiles.column_len(){
                handle.draw_rectangle(row as i32 * self.tile_width + self.row_gap / 2,
                    col as i32 * self.tile_height + self.column_gap / 2,
                    self.tile_width - self.row_gap / 2,
                    self.tile_height - self.column_gap / 2,
                    self.tiles[(row,col)].color
                );
            }
        }
    }
}