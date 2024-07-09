use std::{borrow::BorrowMut, io::BufReader};
use std::io;
use inputs::Input;
use raylib::prelude::*;
use serde::*;
use tiles::{Tile, TileMap};
use array2d::Array2D;


pub mod tiles;
pub mod inputs;
pub mod rhythm;
use rhythm::*;


macro_rules! vec2 {
    ($pair: expr) => {
        Vector2 {x:($pair).0 as f32, y:($pair).1 as f32}
    };

    ($x:expr, $y:expr) => {
        Vector2 {x:($x) as f32, y:($y) as f32}
    };

}

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
    /// where on the grid the player is 
    position: (usize, usize),
    /// how big the player circle is 
    size: f32,
    /// the rhythm the player pulses in
    rhythm: Rhythm,
    /// how large the map is 
    /// TODO: get this out of here
    map_size: (usize,usize),
    last_moved: Option<f64>,
    state: PlayerState
}

impl Player {
    /// How far around the beat you can move
    const MOVEMENT_WINDOW: Sec = 0.15;

    pub fn new(position: (usize, usize), tempo: BPM, map_size: (usize,usize)) -> Self{
        Self {position,size: 1.0,
            rhythm: Rhythm::new(1,tempo,[0]),
            map_size,
            last_moved: None,
            ..Default::default()
            }
    }

    pub fn update(&mut self, delta: Sec, inputs: &[Input]){
        self.rhythm.update(delta);
        for inpt in inputs.iter() {
            if let Input::Key(k) = inpt{
                let direction = match k {
                    KeyboardKey::KEY_W | KeyboardKey::KEY_UP => (0, -1),
                    KeyboardKey::KEY_A | KeyboardKey::KEY_LEFT => (-1, 0),
                    KeyboardKey::KEY_S | KeyboardKey::KEY_DOWN => (0, 1),
                    KeyboardKey::KEY_D | KeyboardKey::KEY_RIGHT => (1,0),
                    _ => (0,0)
                };
                self.move_(vec2!(direction))
            };
        }
        if let Some(pos) = self.last_moved {
            if (self.rhythm.beat() != pos.trunc()) || (self.rhythm.position() < pos) {
                self.last_moved = None;
            }
        }
    }

    /// How large the 
    pub fn size(&self) -> f32 {
        let t = self.rhythm.position().fract();
        let tween = 0.25 * (-1.0 * (8.0 * t).log2().powi(2)).exp() + 1.0;
        (tween * self.size as f64) as f32
    }

    /// Movement
    pub fn move_(&mut self, direction: Vector2){
        let new_position = vec2!(self.position) + direction;
        if self.rhythm.in_window(Player::MOVEMENT_WINDOW) && (self.last_moved.is_none()) {
            if (new_position.x >= 0.0) && (new_position.x < self.map_size.0 as f32){
                self.position.0  = new_position.x as usize
            }
            if (new_position.y >= 0.0) && (new_position.y < self.map_size.1 as f32){
                self.position.1 = new_position.y as usize
            }
            self.last_moved = Some(self.rhythm.position());
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
        (x * (self.tile_width + self.row_gap),
         y * (self.tile_height + self.column_gap)
    )
    }

    pub fn center(&self, x: i32, y: i32) -> (i32, i32) {
        let (xtl, ytl) = self.top_left(x, y);
        (xtl + (self.tile_width - self.row_gap) / 2, ytl + (self.tile_height - self.column_gap) / 2)
    }
}


/// Top-level data structure
pub struct Game{
    level: Option<Level>,
    player: Option<Player>,
    camera: Camera2D,
    dimensions: TileDimensions
}


#[derive(Debug)]
pub enum RCError{
    Array2D(array2d::Error),
    IO(std::io::Error),
    Json(serde_json::Error)
}

impl  From<array2d::Error> for RCError {
    fn from(value: array2d::Error) -> Self {
        RCError::Array2D(value)
    }
}

impl From<io::Error> for RCError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for RCError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl Game {

    pub fn new(camera: Camera2D, dimensions: TileDimensions) -> Self {
        Self {
            level: None, 
            camera,
            player: None,
            dimensions
        }
    }

    pub fn load_level(&mut self, level_path: &str)->Result<(),RCError>{
        let mut level: Level = serde_json::from_reader(BufReader::new(
            std::fs::File::open(level_path)?))?;
        level.set_tempo();
        self.player = Some(Player::new(level.starting_location, level.tempo, level.size_tiles()));
        self.level = Some(level);
        Ok(())
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle){
        if let (Some(level), Some(player)) = (& self.level, &self.player) {
            {
            let mut mode2d = handle.begin_mode2D(self.camera);
                for ((row,col), tile) in level.tiles.enumerate_column_major() {
                    let (x_tl,y_tl) = self.dimensions.top_left(row as i32, col as i32);
                    mode2d.draw_rectangle(x_tl,y_tl,
                        self.dimensions.tile_width,
                        self.dimensions.tile_height,
                        tile.get_color()
                    );
                }
                let (player_x,player_y)  = self.dimensions.center(
                    player.position.0 as i32, player.position.1 as i32);
                let player_radius = self.dimensions.tile_height as f32 * player.size() / 3.0;
                mode2d.draw_circle(player_x, player_y, 
                    player_radius, Color::YELLOW);
            }
        
            let (rows, columns) = level.size_tiles();
            let height = (rows as i32) * self.dimensions.tile_height;
            let width = (columns as i32) * self.dimensions.tile_width;
            let (height, width) = (height as f64, width as f64);
            let mut draw_msg_box = || {
                handle.draw_rectangle(
                    (0.2 * width) as i32, 
                    (0.2 * height) as i32, 
                    (0.4 * width) as i32, 
                    (0.2 * height) as i32, Color::GRAY);
            };
            
            match player.state {
                PlayerState::Cleared => {
                    draw_msg_box();
                    handle.draw_text("Level cleared!", 3 * width as i32/ 10, 
                    3 * height as i32 * 10, 18, Color::BLACK)
        
                },
                PlayerState::Died => {
                    draw_msg_box();
                    handle.draw_text("You died", 3 * width as i32/ 10, 3 * height as i32 / 10, 18, Color::BLACK)
                }, // need to implement this
                PlayerState::Playing => {}
            }
        }
    }

    pub fn update(&mut self, delta:f64, inputs:&[Input]){
        if let (Some(level), Some(player)) = (&mut self.level, &mut self.player) {
            level.update(delta, inputs);
            match player.state{
            PlayerState::Playing => {
                player.update(delta, inputs);
                let (row, col) = player.position;
                match level.tiles.get(row,col){
                    None => {player.state = PlayerState::Died}
                    Some(tile) => {
                        if tile.goal {
                            player.state = PlayerState::Cleared;
                        } else if tile.rhythm.as_ref().is_some() {
                            if !tile.on(Level::WINDOW + 0.1).unwrap(){
                                player.state = PlayerState::Died;
                            }
                        } 
                    }
                }
            },
            PlayerState::Died=> {
                if inputs.iter().any(|i|{
                    matches!(i,Input::Key(KeyboardKey::KEY_R))
                }){
                    self.reset()
                }
            },
            _ => {}
        }
    }
    }

    fn reset(&mut self) {
        if let (Some(level), Some(player)) = (&mut self.level, &mut self.player) {
            player.position = level.starting_location;
            player.state = PlayerState::Playing;
            player.rhythm.reset();
            for t in level.tiles.iter_mut(){
                if let Some(r) = t.rhythm.as_mut(){
                    r.reset();
                }
            }
    
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    #[serde(flatten)]
    pub tiles: tiles::TileMap,
    pub starting_location: (usize, usize),
    pub tempo: BPM,
}

impl Level {
    const WINDOW: f64 = 0.05;
    pub fn new(mut tiles: Array2D<Tile>, starting_location: (usize, usize), tempo: BPM) -> Self{
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
            starting_location,
            tempo: tempo
        }
    }

    pub fn set_tempo(&mut self){
        for tile in self.tiles.iter_mut() {
            if let Some(r) = &mut tile.rhythm {
                r.duration = beat_length(self.tempo);
            }
        }
    }

    pub fn size_tiles(&self) -> (usize, usize) {
        (self.tiles.num_rows(), self.tiles.num_columns())
    }

    pub fn update(&mut self, delta: Sec, _inputs: &[Input]){
        for tile in self.tiles.iter_mut(){
            tile.update(delta)
        }
    }
    
}

