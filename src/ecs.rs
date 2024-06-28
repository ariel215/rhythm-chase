use std::{cell::RefCell, collections::HashSet, io::{BufReader, Result}, rc::Rc, sync::Arc};

use math::Vector2;
use raylib::*;
use crate::*;



#[derive(Debug, Default,Clone)]
pub struct Position(Vector2);


#[derive(Default, Clone)]
pub struct Components{
    pub positions: Vec<Position>,
    pub rhythms: Vec<Option<Rhythm>>,
    pub players: Vec<Option<Player>>,
    pub tiles: Vec<Option<Tile>>,
    pub camera: Vec<Option<Camera2D>>,
    free_entities: HashSet<usize>
}


pub struct Game{
    components: Components,
    dimensions: TileDimensions,
    map: Option<TileMap>
}

pub struct Entity(usize);

////////////////////////////////////////
//Systems
////////////////////////////////////////
impl Components{

    pub fn new() -> Self {
        Components{
            positions: vec![],
            rhythms: vec![],
            players: vec![],
            tiles: vec![],
            camera: vec![],
            free_entities: HashSet::new()
        }
    }

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
                self.camera.push(None);
                entity
            }
        }
    }

    pub fn clear_entity(&mut self, entity: Entity){
        self.positions[entity.0] = Position(Vector2::default());
        self.rhythms[entity.0] = None;
        self.players[entity.0] = None;
        self.tiles[entity.0] = None;
        self.camera[entity.0] = None;
        self.free_entities.insert(entity.0);
    }
}

pub fn move_player(components: &mut Components, player_entity: Entity, direction: Position){
    let player = components.players[player_entity.0].unwrap();
    match components.rhythms[player_entity.0]{
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
            handle.draw_rectangle_v(position.0, 
                vec2!(dimensions.tile_width, dimensions.tile_height),
                tile.get_color())
        }
}

pub fn draw_player<T: RaylibDraw>(components: & Components, handle: &mut RaylibMode2D<T>){
    for (entity, maybe_player) in components.players.iter().enumerate(){
        if let Some(player) = maybe_player{
            let position = components.positions[entity].0;
            handle.draw_circle_v(position, player.size(), Player::COLOR)
        }
    }
}

pub fn update_rhythms(components: &mut Components, delta_t: Sec){
    components.rhythms.iter_mut().map(|r| r.as_mut().map(|rhythm| rhythm.update(delta_t)));
}

impl Game{
    fn clear_map(&mut self){
        let tile_entites = self.components.tiles.iter().enumerate().filter_map(
            |(ent,tile)| {tile.map(|_t|{Entity(ent)})}
        );
        for entity in tile_entites{
            self.components.clear_entity(entity)
        }
    }


    fn load_map(&mut self, map: &str)->Result<()>{
        if self.map.is_some(){
            self.clear_map();
        }

        let map_file = std::fs::File::open(map)?;
        let map: TileMap = serde_json::from_reader(BufReader::new(map_file))?;
        for ((r,c), tile ) in map.enumerate_column_major(){
            let entity = self.components.new_entity();
            let position = vec2!(self.dimensions.top_left(r as i32, c as i32));
            self.components.positions[entity.0] = Position(position);
            self.components.tiles[entity.0] = Some(tile.clone());
            self.components.rhythms[entity.0] = tile.rhythm.clone();
        }
        self.map = Some(map);
        Ok(())
    }

}