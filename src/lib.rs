use std::{collections::HashSet, hash::Hash, str::FromStr, usize};
use mki::Keyboard;
use raylib::prelude::*;
use array2d::Array2D;
use serde::*;
use std::cmp::Eq;

#[derive(Debug, Default, PartialEq, Clone,Serialize,Deserialize)]
pub struct Tile {
    color: Color, 
    rhythm: Option<TileRhythm>
    // todo: add more features
}

impl Eq for Tile {
    
}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.color_to_int().hash(state);
        self.rhythm.hash(state);
    }
}


impl Tile {
    pub fn from(color: &Color, rhythm: Option<TileRhythm> ) -> Self{
        Tile {
            color: *color,
            rhythm,
        }
    }
    pub fn update(&mut self, delta: Sec){
        if let Some(rhythm) = &mut self.rhythm  {
            rhythm.update(delta)
        }
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


#[derive(Debug,Default,Clone,PartialEq, Serialize,Deserialize)]
pub struct TileRhythm {
    // Number of beats in a measure
    length: usize,
    // Length of a beat, in seconds
    duration: Sec,
    // which beats to play; zero-indexed
    beats: HashSet<usize>,
    //
    #[serde(skip)]
    time: Sec,
}

fn canonical(v: f64) -> i64{
    (v*1024.0*1024.0).round() as i64
}

impl Hash for TileRhythm{

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        canonical(self.duration).hash(state);
        for b in &self.beats{
            b.hash(state);
        }
        canonical(self.time).hash(state);
    }
}

impl TileRhythm{
    pub fn update(&mut self, delta: Sec){
        self.time += delta;
        self.time %= self.duration * self.length as Sec;
    }

    pub fn on(&self) -> bool {
        self.beats.contains(& (self.position().trunc() as usize))
    }

    pub fn position(&self) -> f64 {
        self.time / self.duration
    }

    pub fn beat(&self) -> f64 {
        self.position().trunc()
    }

    pub fn in_window(&self, window_size: Sec)-> bool {
        if self.beats.contains(&0) && (self.length as f64 - self.position()) < (window_size / self.duration) {
            return true
        } 

        self.beats.iter().any(
            |beat| ((*beat as f64) - self.position()).abs() < (window_size / self.duration)
        )
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
    assert!((1.15 - r.position()) < 1e-6);
    r.update(r.duration);
    assert_eq!(r.on(), false);
    assert!((r.time - 0.15).abs() < 1e-6);
}

#[test]
fn rhythm_window(){
    let mut tr = TileRhythm::new(2,120., vec![0]);
    assert!(tr.in_window(1e-6));
    tr.update(0.04);
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));
    tr.update(0.5 );
    assert!(tr.in_window(0.05));


    let mut tr = TileRhythm::new(4, 120., vec![0,2]);
    tr.update(0.04);
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));
    tr.update(0.5 );
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));

}


#[derive(Default,Debug, Clone)]
pub struct Player{
    position: (usize, usize),
    size: f32,
    rhythm: TileRhythm,
    map_size: (usize,usize),
    movement_window: Sec,
    keys_to_check: Vec<Keyboard>,
    last_moved: Option<f64>
}

impl Player {
    const KEYS_TO_CHECK: [&'static str; 8] = ["W","A","S","D", "Up", "Left", "Right", "Down"];

    pub fn new(position: (usize, usize), tempo: BPM, map_size: (usize,usize), window: Sec) -> Self{
        Self {position,size: 1.0,
            rhythm: TileRhythm::new(1,tempo,[0]),
            map_size,
            movement_window: window,
            keys_to_check: Self::KEYS_TO_CHECK.iter().map(|s|mki::Keyboard::from_str(s).unwrap()).collect::<Vec<_>>(),
            last_moved: None
            }
    }

    pub fn update(&mut self, delta: Sec){
        self.rhythm.update(delta);
        let keys_pressed = self.keys_to_check.iter().filter_map(|k|if mki::are_pressed(&[*k]) {Some(*k)} else {None}).collect::<Vec<_>>();
        for key in keys_pressed {
            self.move_(
            match key {
                Keyboard::W | Keyboard::Up => (0, -1),
                Keyboard::A | Keyboard::Left => (-1, 0),
                Keyboard::S | Keyboard::Down => (0, 1),
                Keyboard::D | Keyboard::Right => (1,0),
                _ => (0,0)
            });
        }
        if let Some(pos) = self.last_moved {
            if (self.rhythm.beat() != pos.trunc()) || (self.rhythm.position() < pos) {
                self.last_moved = None;
            }
        }
    }

    pub fn size(&self) -> f32 {
        let t = self.rhythm.position().fract();
        let tween = 0.25 * (-1.0 * (8.0 * t).log2().powi(2)).exp() + 1.0;
        (tween * self.size as f64) as f32
    }

    pub fn move_(&mut self, direction: (isize, isize)){
        let new_x = self.position.0 as isize + direction.0;
        let new_y = self.position.1  as isize + direction.1;
        if self.rhythm.in_window(self.movement_window) && (self.last_moved.is_none()) {
            if (new_x >= 0) && (new_x < self.map_size.0 as isize){
                self.position.0 = new_x as usize;
            }
            if (new_y >= 0) && (new_y < self.map_size.1 as isize){
                self.position.1 = new_y as usize;
            }
            self.last_moved = Some(self.rhythm.position());
        }
    }

}


#[derive(Debug,Serialize,Deserialize)]
pub struct TileDimensions {
    tile_width: i32,
    tile_height: i32,
    row_gap: i32,
    column_gap: i32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    #[serde(flatten)]
    pub tiles: TileMap,
    pub dimensions: TileDimensions,
    #[serde(skip)]
    pub player: Player,
    // todo: add more features
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMap{
    tiles: Vec<Tile>,
    map: Array2D<usize>,
}

impl From<&Array2D<Tile>>for TileMap {
    fn from(tiles: &Array2D<Tile>) -> Self {
        let tileset: HashSet<_> = tiles.elements_row_major_iter().collect();
        let tiles_with_indices: Vec<_> = tileset.into_iter().collect();
        let tilemap = Array2D::from_iter_row_major(tiles.elements_row_major_iter().map(
            |t | {
                for (i,t1) in tiles_with_indices.iter().enumerate(){
                    if &t == t1 {
                        return i
                    }
                }
                panic!()
            }
        ),
        tiles.num_rows(),
        tiles.num_columns()
        ).unwrap();
        Self {
            tiles: tiles_with_indices.into_iter().cloned().collect(),
            map: tilemap,
        }
    }
}


impl TileMap{
    fn enumerate_column_major(&self) -> impl Iterator<Item=((usize,usize),&Tile)>{
        self.map.enumerate_column_major().map(
            |((r,c),idx)| ((r,c), &self.tiles[*idx])
        )
    }

    fn indices_column_major(&self) -> impl Iterator<Item=(usize,usize)>{
        self.map.indices_column_major()
    }

    fn get(&self, r: usize, c:usize) -> Option<&Tile>{
        let idx = self.map.get(r,c);
        idx.map(|i| &self.tiles[*i])
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Tile>{
        let idx = self.map.get(r,c);
        idx.map(|i| &mut self.tiles[*i])
    }
}


impl Level {

    pub fn new(tiles: Array2D<Tile>, tile_width: i32, tile_height: i32, row_gap: i32, column_gap: i32) -> Self{
        let duration = tiles.get_column_major(0).as_ref().and_then(|t| t.rhythm.as_ref()).map(|tr| tr.duration).unwrap_or(0.);
        let map_size = (tiles.row_len(), tiles.column_len());
        Level {
            tiles: <&Array2D<Tile> as Into<TileMap>>::into(&tiles),
            dimensions: TileDimensions {tile_width,
            tile_height,
            row_gap,
            column_gap},
            player: Player::new((1,1),beat_length(duration), map_size,0.05)
        }
    }

    pub fn size(&self) -> (usize, usize){
        (self.tiles.map.num_rows(), self.tiles.map.num_columns())
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle){
        for ((row,col), tile) in self.tiles.enumerate_column_major() {
            let (x_tl,y_tl) = self.dimensions.top_left(row as i32, col as i32);
            let (x_br, y_br) = self.dimensions.bottom_right(row as i32, col as i32);
            handle.draw_rectangle(x_tl,y_tl,
                x_br - x_tl,
                y_br - y_tl,
                tile.get_color()
            );
        }
        let (player_x,player_y)  = self.dimensions.center(
            self.player.position.0 as i32, self.player.position.1 as i32);
        let player_radius = self.dimensions.tile_height as f32 * self.player.size() / 3.0;
        handle.draw_circle(player_x, player_y, 
            player_radius, Color::YELLOW)
    }


    pub fn update(&mut self, delta: Sec){
        for tile in self.tiles.tiles.iter_mut(){
            tile.update(delta)
        }
        self.player.update(delta);
    }
}

