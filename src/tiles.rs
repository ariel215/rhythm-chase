use crate::{rhythm::*, vec2};
use std::{collections::HashSet,hash::Hash};
use raylib::prelude::*;
use serde::*;
use array2d::Array2D;


#[derive(Debug, Default, PartialEq, Clone,Serialize,Deserialize)]
pub struct Tile {
    pub color: Color, 
    pub rhythm: Option<Rhythm>,
    #[serde(default = "default_goal")]
    pub goal: bool
    // todo: add more features?
}

fn default_goal()->bool{
    false
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.color_to_int().hash(state);
        self.rhythm.hash(state);
    }
}

impl Tile {
    pub fn from(color: &Color, rhythm: Option<Rhythm> ) -> Self{
        Tile {
            color: *color,
            rhythm,
            ..Default::default()
        }
    }
    pub fn update(&mut self, delta: Sec){
        if let Some(rhythm) = &mut self.rhythm  {
            rhythm.update(delta)
        }
    }

    pub fn on(&self, window_size: Sec) -> Option<bool> {
        self.rhythm.as_ref().map(|r|{
            r.on_beat(window_size)
        })
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

#[test]
fn test_tile_on(){
    let rhyth = Rhythm::new(2, 60.0, [0]);
    let mut t = Tile{
        color: Color::WHITE,
        rhythm: Some(rhyth),
        goal: false
    };
    assert!(t.on(0.015).unwrap());
    t.rhythm.as_mut().map(|r| r.update(1.005));
    assert!(t.on(0.015).unwrap());
    t.rhythm.as_mut().map(|r| r.update(0.015));
    assert!(!t.on(0.015).unwrap())


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMap{
    tiles: Vec<Tile>,
    map: Array2D<usize>,
    pub starting_location: Vector2
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
            starting_location: vec2!((0,0))
        }
    }
}


impl TileMap{

    pub fn enumerate_column_major(&self) -> impl Iterator<Item=((usize,usize),&Tile)>{
        self.map.enumerate_column_major().map(
            |((r,c),idx)| ((r,c), &self.tiles[*idx])
        )
    }

    pub fn indices_column_major(&self) -> impl Iterator<Item=(usize,usize)>{
        self.map.indices_column_major()
    }

    pub fn get(&self, r: usize, c:usize) -> Option<&Tile>{
        let idx = self.map.get(r,c);
        idx.map(|i| &self.tiles[*i])
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Tile>{
        let idx = self.map.get(r,c);
        idx.map(|i| &mut self.tiles[*i])
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile>{
        self.map.elements_column_major_iter().map(|idx| &self.tiles[*idx])
    }

    pub fn num_rows(&self)->usize{
        self.map.num_rows()
    }

    pub fn num_columns(&self) -> usize{
        self.map.num_columns()
    } 

}