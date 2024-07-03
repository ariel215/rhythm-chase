use std::{cmp::min, collections::{HashMap, HashSet}, io::{BufReader, Result}};
use io::Write;
use itertools::{multizip, Itertools};

use math::Vector2;
use raylib::*;
use crate::*;
use tiles::*;

#[derive(Debug, Default,Clone)]
pub struct Position(pub Vector2);

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


#[derive(Debug, Default)]
pub struct Components{
    pub positions: Vec<Position>,
    pub rhythms: Vec<Option<Rhythm>>,
    pub players: Vec<Option<Player>>,
    pub tiles: Vec<Option<Tile>>,
    pub areas: Vec<Option<Rectangle>>,
    free_entities: HashSet<usize>
}

 

pub struct Entity(pub usize);

////////////////////////////////////////
//Systems
////////////////////////////////////////
impl Components{

    pub fn new_entity(&mut self)->Entity{    
        let existing = self.free_entities.iter().next();
        match existing {
            Some(entity) => Entity(*entity),
            None => {
                let entity = Entity(self.positions.len());
                self.positions.push(Position::default());
                self.rhythms.push(None);
                self.players.push(None);
                self.tiles.push(None);
                self.areas.push(None);
                entity
            }
        }
    }

    pub fn clear_entity(&mut self, entity: Entity){
        self.positions[entity.0] = Position(Vector2::default());
        self.rhythms[entity.0] = None;
        self.players[entity.0] = None;
        self.tiles[entity.0] = None;
        self.areas[entity.0] = None;
        self.free_entities.insert(entity.0);
    }

    pub fn tie(&self) -> impl Iterator<Item = (&Position, &Option<Rhythm>, &Option<Player>, &Option<Tile>, &Option<Rectangle>)> {
        multizip((self.positions.iter(),self.rhythms.iter(), self.players.iter(), self.tiles.iter(),
        self.areas.iter()))
    }

    pub fn tie_mut(&mut self) -> impl Iterator<Item = (&mut Position, &mut Option<Rhythm>, &mut Option<Player>, &mut Option<Tile>, &mut Option<Rectangle>)>{
        multizip((self.positions.iter_mut(),self.rhythms.iter_mut(), self.players.iter_mut(), self.tiles.iter_mut(),
        self.areas.iter_mut()))
    }

}

fn find<T>(elts: &[Option<T>])->impl Iterator<Item = &T>{
    elts.iter().filter_map(|elt|elt.as_ref())
}

fn ifind<T>(elts: &[Option<T>])-> impl Iterator<Item = (usize, &T)>{
    elts.iter().enumerate().filter_map(|(i,elt)|elt.as_ref().map(|e|(i,e)))

}


fn find_mut<T>(elts: &mut[Option<T>]) -> impl Iterator<Item=&mut T>{
    elts.iter_mut().filter_map(|elt|elt.as_mut())

}

fn ifind_mut<T>(elts: &mut [Option<T>])-> impl Iterator<Item = (usize, &mut T)>{
    elts.iter_mut().enumerate()
    .filter_map(|(i,elt)|elt.as_mut().map(|e|(i,e)))

}


pub fn move_player(components: &mut Components, player_entity: Entity, direction: Position){
    let player = components.players[player_entity.0].as_ref().unwrap();
    match &components.rhythms[player_entity.0]{
        Some(rhythm) => {
            if rhythm.in_window(player.movement_window){
                components.positions[player_entity.0].0 += direction.0;
            }
        },
        None => {components.positions[player_entity.0].0 += direction.0;}
    }
}

fn get_tile_color(tile: &Tile, rhythm: &Option<Rhythm>)->Color{
    match rhythm {
        Some(rhythm) => {
            if rhythm.on() { tile.color } else {Color::WHITE}
        }
        None => tile.color
    }
}

pub fn draw_tiles<T: RaylibDraw>(components: & Components, dimensions: &TileDimensions, handle: &mut RaylibMode2D<T>) {
    for (entity, tile) in ifind(&components.tiles){
            let position = &components.positions[entity];
            let rhythm = &components.rhythms[entity];
            let color = get_tile_color(tile, rhythm);
            handle.draw_rectangle_v(position.0, 
                vec2!(dimensions.tile_width, dimensions.tile_height),
                color)
        }
}

pub fn draw_player<T: RaylibDraw>(components: & Components, dimensions: &TileDimensions, handle: &mut RaylibMode2D<T>){
    for (entity,player) in ifind(&components.players){
        let position = components.positions[entity].0;
        let tile_size = min(dimensions.tile_height, dimensions.tile_width) as f64;
        handle.draw_circle_v(position, (player.size * tile_size / 2.5) as f32, Player::COLOR)
    }
}

pub fn update_rhythms(components: &mut Components, delta_t: Sec){
    for rhythm in find_mut(&mut components.rhythms){
        rhythm.update(delta_t);
    }
}


fn movement_direction(input: &Input) -> Vector2{
    if let Input::Key(k) = input {
        match k {
            KeyboardKey::KEY_A | KeyboardKey::KEY_LEFT => { vec2!(-1,0)},
            KeyboardKey::KEY_W | KeyboardKey::KEY_UP => {vec2!(0,-1)},
            KeyboardKey::KEY_S | KeyboardKey::KEY_DOWN => {vec2!(0,1)},
            KeyboardKey::KEY_D | KeyboardKey::KEY_RIGHT => {vec2!(1,0)},
            _ => Vector2::zero()
        }
    } else { Vector2::zero() }
}

pub fn update_players(components: &mut Components, dimensions: &TileDimensions, _delta_t: Sec, inputs:&[Input]){
    let step_size = vec2!(dimensions.tile_width+dimensions.column_gap, dimensions.tile_height + dimensions.row_gap);
    let directions = inputs.iter().map(|i|step_size * movement_direction(i)).collect_vec();
    for (entity, player) in ifind_mut(&mut components.players){
        let rhythm = components.rhythms[entity].as_ref().unwrap();
        let position = &mut components.positions[entity];
        let t = rhythm.position().fract();
        player.size =  0.25 * (-1.0 * (8.0 * t).log2().powi(2)).exp() + 1.0;
        for direction in directions.iter() {
            position.0 += *direction;
        }
    }
}
