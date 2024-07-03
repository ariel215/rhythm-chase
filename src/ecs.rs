use std::{collections::{HashMap, HashSet}, io::{BufReader, Result}};
use itertools::multizip;

use math::Vector2;
use raylib::*;
use crate::*;

#[derive(Debug, Default,Clone)]
pub struct Position(Vector2);

#[derive(Debug, Default)]
pub struct Components{
    pub positions: Vec<Position>,
    pub rhythms: Vec<Option<Rhythm>>,
    pub players: Vec<Option<Player>>,
    pub tiles: Vec<Option<Tile>>,
    pub areas: Vec<Option<Rectangle>>,
    free_entities: HashSet<usize>
}

 
pub struct Game{
    components: Components,
    dimensions: Option<TileDimensions>,
    map: Option<TileMap>,
    tempo: rhythm::BPM,
    camera: Option<Camera2D>
}

pub struct Entity(usize);

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


pub fn draw_tiles<T: RaylibDraw>(components: & Components, dimensions: &TileDimensions, handle: &mut RaylibMode2D<T>) {
    for (entity, tile) in components.tiles.iter().enumerate().filter_map(
        |(ent, mt)| if mt.is_some() {Some((ent,mt.as_ref().unwrap()))} else {None}
    ){
            let position = &components.positions[entity];
            let mut color = tile.get_color();
            if let Some(rhythm) = components.rhythms[entity].as_ref(){
                if !rhythm.on(){
                    color = Color::WHITE;
                }
            }
            handle.draw_rectangle_v(position.0, 
                vec2!(dimensions.tile_width, dimensions.tile_height),
                color)
        }
}

pub fn draw_player<T: RaylibDraw>(components: & Components, handle: &mut RaylibMode2D<T>){
    for (entity, maybe_player) in components.players.iter().enumerate(){
        if let Some(player) = maybe_player{
            let position = components.positions[entity].0;
            handle.draw_circle_v(position, player.size as f32, Player::COLOR)
        }
    }
}

pub fn update_rhythms(components: &mut Components, delta_t: Sec){
    for rhythm in components.rhythms.iter_mut().filter_map(|r|r.as_mut()){
        rhythm.update(delta_t);
    }
}

pub fn update_players(components: &mut Components, _delta_t: Sec){
    for (entity, player) in components.players.iter_mut().enumerate()
    .filter_map(|(i,mp)|mp.as_mut().map(|p|(i,p))){
        let rhythm = components.rhythms[entity].as_ref().unwrap();
        let t = rhythm.position().fract();
        player.size =  0.25 * (-1.0 * (8.0 * t).log2().powi(2)).exp() + 1.0;
    }
}

impl Game{
    fn clear_map(&mut self){
        for (position, rhythm, _player, tile,area) in self.components.tie_mut(){
                if tile.is_some(){
                    *position = Position::default();
                    *rhythm = None;
                    *area = None;
                    *tile = None;        
                }
        }
    }

    pub fn new(tempo: BPM, tile_dimensions: TileDimensions)->Self{
        Self{
            tempo,
            components: Components::default(),
            dimensions: Some(tile_dimensions),
            map: None,
            camera: None
        }
    }

    pub fn load_map(&mut self, map: &str)->Result<()>{
        if self.map.is_some(){
            self.clear_map();
        }

        let dimensions = self.dimensions.as_ref().ok_or(std::io::Error::other("no dimensions"))?;
        if self.tempo == 0.0 {return Result::Err(std::io::Error::other("no tempo"));}

        let map_file = std::fs::File::open(map)?;
        let map: TileMap = serde_json::from_reader(BufReader::new(map_file))?;
        for ((r,c), tile ) in map.enumerate_column_major(){
            let entity = self.components.new_entity();
            let position = vec2!(dimensions.top_left(r as i32, c as i32));
            self.components.positions[entity.0] = Position(position);
            self.components.tiles[entity.0] = Some(tile.clone());
            let rhythm = tile.rhythm.clone().map(|mut rh| {
                rh.set_tempo(self.tempo); rh});
            self.components.rhythms[entity.0] = rhythm;
        }
        self.map = Some(map);
        Ok(())
    }

    pub fn set_dimensions(&mut self, dimensions: TileDimensions)->Self{
        self.dimensions = Some(dimensions);
    }

    pub fn add_player(&mut self, player: Player)->Option<Entity>{
        if let Some(map) = &self.map{
            let player_ent = self.components.new_entity();
            self.components.players[player_ent.0] = Some(player);
            self.components.positions[player_ent.0] = Position(map.starting_location);
            self.components.rhythms[player_ent.0] = Some(Rhythm::new(1, self.tempo, [0]));
            Some(player_ent)
        } else { None }
    }


    pub fn add_camera(&mut self, camera: Camera2D){
        self.camera = Some(camera);
    }

    pub fn set_tempo(&mut self, bpm: BPM){
        self.tempo = bpm;
    }

    pub fn update(&mut self, delta_t: Sec, _inputs: &[Input]){
        update_players(&mut self.components, delta_t);
        update_rhythms(&mut self.components, delta_t)
    }

    pub fn draw<T: RaylibDraw>(&self, handle: &mut T)->std::result::Result<(),()>{
        let mut handle2d = handle.begin_mode2D(self.camera.ok_or(())?);
        draw_tiles(&self.components, self.dimensions.as_ref().ok_or(())?, &mut handle2d);
        draw_player(&self.components, &mut handle2d);
        Ok(())
    }

}