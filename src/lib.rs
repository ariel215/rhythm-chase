use std::borrow::BorrowMut;

use inputs::Input;
use raylib::prelude::*;
use serde::*;
use tiles::{Tile, TileMap};
use array2d::Array2D;


pub mod tiles;
pub mod inputs;
pub mod rhythm;
use rhythm::*;
pub mod ecs;
mod macros;

/// What states the player can be in
#[derive(Debug, Default,Clone, Copy)]
enum PlayerState {
    #[default]
    Playing,
    Cleared,
    Died
}


/// The player game object
#[derive(Default,Debug, Clone)]
pub struct Player{
    /// how big the player circle is 
    size: f64,
    /// How far around the beat you can move
    movement_window: Sec,
    last_moved: Option<f64>,
    state: PlayerState
}

impl Player {
    const COLOR: Color = Color::YELLOW;

    pub fn new(window: Sec) -> Self{
        Self {
            size: 1.0,
            movement_window: window,
            last_moved: None,
            ..Default::default()
        }
    }
}


#[derive(Debug,Serialize,Deserialize)]
pub struct TileDimensions {
    pub tile_width: i32,
    pub tile_height: i32,
    pub row_gap: i32,
    pub column_gap: i32,
}

impl TileDimensions {
    pub fn top_left(&self, x: i32, y: i32) -> (i32,i32){
        (x * self.tile_width + self.row_gap / 2,
         y * self.tile_height + self.column_gap / 2)
    }

    pub fn bottom_right(&self, x:i32, y: i32) -> (i32,i32){
        ((x+1) * self.tile_width - self.row_gap,
         (y+1) * self.tile_height - self.column_gap)
    }

    pub fn center(&self, x: i32, y: i32) -> (i32, i32) {
        let (xtl, ytl) = self.top_left(x, y);
        (xtl + (self.tile_width - self.row_gap) / 2, ytl + (self.tile_height - self.column_gap) / 2)
    }
}


// /// Top-level data structure
// pub struct Game{
//     level: Level,
//     player: Player,
//     camera: Camera2D
// }

// impl Game {

//     pub fn new(level: Level, mut player: Player, camera: Camera2D) -> Self {
//         player.position = level.starting_location;
//         Self {
//             level, 
//             camera,
//             player
//         }
//     }

//     pub fn set_level(&mut self, new_level: Level){
//         self.level = new_level;
//     }

//     pub fn draw(&self, handle: &mut RaylibDrawHandle){
//             {
//             let mut mode2d = handle.begin_mode2D(self.camera);
//             // let mode2d = handle;
//                 for ((row,col), tile) in self.level.tiles.enumerate_column_major() {
//                     let (x_tl,y_tl) = self.level.dimensions.top_left(row as i32, col as i32);
//                     let (x_br, y_br) = self.level.dimensions.bottom_right(row as i32, col as i32);
//                     mode2d.draw_rectangle(x_tl,y_tl,
//                         x_br - x_tl,
//                         y_br - y_tl,
//                         tile.get_color()
//                     );
//                 }
//                 let (player_x,player_y)  = self.level.dimensions.center(
//                     self.player.position.0 as i32, self.player.position.1 as i32);
//                 let player_radius = self.level.dimensions.tile_height as f32 * self.player.size() / 3.0;
//                 mode2d.draw_circle(player_x, player_y, 
//                     player_radius, Color::YELLOW);
//             }
        
//             let (rows, columns) = self.level.size_tiles();
//             let height = (rows as i32) * self.level.dimensions.tile_height;
//             let width = (columns as i32) * self.level.dimensions.tile_width;
//             let (height, width) = (height as f64, width as f64);
//             let mut draw_msg_box = || {
//                 handle.draw_rectangle(
//                     (0.2 * width) as i32, 
//                     (0.2 * height) as i32, 
//                     (0.4 * width) as i32, 
//                     (0.2 * height) as i32, Color::GRAY);
//             };
            
//             match self.player.state {
//                 PlayerState::Cleared => {
//                     draw_msg_box();
//                     handle.draw_text("Level cleared!", 3 * width as i32/ 10, 
//                     3 * height as i32 * 10, 18, Color::BLACK)
        
//                 },
//                 PlayerState::Died => {
//                     draw_msg_box();
//                     handle.draw_text("You died", 3 * width as i32/ 10, 3 * height as i32 / 10, 18, Color::BLACK)
//                 }, // need to implement this
//                 PlayerState::Playing => {}
//             }
    
//     }

//     pub fn update(&mut self, delta:f64, inputs:&[Input]){
//         self.level.update(delta, inputs);
//         match self.player.state{
//             PlayerState::Playing => {
//                 self.player.update(delta, inputs);
//                 let (row, col) = self.player.position;
//                 match self.level.tiles.get(row,col){
//                     None => {self.player.state = PlayerState::Died}
//                     Some(tile) => {
//                         if tile.goal {
//                             self.player.state = PlayerState::Cleared;
//                         } else if tile.rhythm.as_ref().is_some() {
//                             if !tile.on(Level::WINDOW + 0.1).unwrap(){
//                                 self.player.state = PlayerState::Died;
//                             }
//                         } 
//                     }
//                 }
//             },
//             PlayerState::Died=> {
//                 if inputs.iter().any(|i|{
//                     matches!(i,Input::Key(KeyboardKey::KEY_R))
//                 }){
//                     self.reset()
//                 }
//             },
//             _ => {}
//         }
//     }

//     fn reset(&mut self) {
//         self.player.position = self.level.starting_location;
//         self.player.state = PlayerState::Playing;
//         self.player.rhythm.reset();
//         for t in self.level.tiles.iter_mut(){
//             if let Some(r) = t.rhythm.as_mut(){
//                 r.reset();
//             }
//         }
//     }
// }



#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    #[serde(flatten)]
    pub tiles: tiles::TileMap,
    pub dimensions: TileDimensions,
    pub starting_location: (usize, usize),
    pub tempo: BPM,
}



impl Level {
    const WINDOW: f64 = 0.05;
    pub fn new(mut tiles: Array2D<Tile>, tile_width: i32, tile_height: i32, row_gap: i32, column_gap: i32, starting_location: (usize, usize), tempo: BPM) -> Self{
        for i in 0..tiles.num_rows() {
            for j in 0..tiles.num_columns(){
                if let Some(tile) = tiles.get_mut(i, j){
                    if let Some(rhythm) = &mut tile.rhythm{
                        rhythm.set_tempo(tempo);
                    }
                }
            }
        }

        Level {
            tiles: <&Array2D<Tile> as Into<TileMap>>::into(&tiles),
            dimensions: TileDimensions {tile_width,
            tile_height,
            row_gap,
            column_gap},
            starting_location,
            tempo: tempo
        }
    }

    pub fn size_tiles(&self) -> (usize, usize) {
        (self.tiles.num_rows(), self.tiles.num_columns())
    }

    pub fn size_pixels(&self) -> Vector2 {
        let size = vec2!(self.size_tiles());
        size * vec2!(self.dimensions.tile_height, self.dimensions.tile_width) 
            + (size + Vector2::one()) * vec2!(self.dimensions.row_gap,self.dimensions.column_gap)
    }


    // pub fn update(&mut self, delta: Sec, _inputs: &[Input]){
    //     for tile in self.tiles.iter_mut(){
    //         tile.update(delta)
    //     }
    // }
    
}

