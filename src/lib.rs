use std::io;
use inputs::Input;
use raylib::prelude::*;

pub mod tiles;
pub mod inputs;
pub mod rhythm;
pub mod ecs;

use crate::{rhythm::*, ecs::*, tiles::*};

mod macros;



// /// Top-level data structure
pub struct Game{
    components: Components,
    dimensions: Option<TileDimensions>,
    map: Option<TileMap>,
    tempo: rhythm::BPM,
    camera: Option<Camera2D>
}

impl Game{

    pub fn new(tempo: BPM, tile_dimensions: TileDimensions)->Self{
        Self{
            tempo,
            components: Components::default(),
            dimensions: Some(tile_dimensions),
            map: None,
            camera: None
        }
    }

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


    pub fn load_map(&mut self, map: &str)->io::Result<()>{
        if self.map.is_some(){
            self.clear_map();
        }

        let dimensions = self.dimensions.as_ref().ok_or(std::io::Error::other("no dimensions"))?;
        if self.tempo == 0.0 {return Result::Err(std::io::Error::other("no tempo"));}

        let map_file = std::fs::File::open(map)?;
        let map: TileMap = serde_json::from_reader(io::BufReader::new(map_file))?;
        for ((r,c), tile ) in map.enumerate_column_major(){
            let entity = self.components.new_entity();
            let position = vec2!(dimensions.top_left(r as i32, c as i32));
            dbg!(position);
            self.components.positions[entity.0] = Position(position);
            self.components.tiles[entity.0] = Some(tile.clone());
            let rhythm = tile.rhythm.clone().map(|mut rh| {
                rh.set_tempo(self.tempo); rh});
            self.components.rhythms[entity.0] = rhythm;
        }
        self.map = Some(map);
        Ok(())
    }

    pub fn add_player(&mut self, player: Player)->Option<Entity>{
        if let (Some(map), Some(dimensions)) = (&self.map, &self.dimensions){
            let player_ent = self.components.new_entity();
            self.components.players[player_ent.0] = Some(player);
            self.components.positions[player_ent.0] = Position(
                vec2!(dimensions.center(map.starting_location.x as i32, map.starting_location.y as i32))
            );
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

    pub fn update(&mut self, delta_t: Sec, inputs: &[Input]){
        update_rhythms(&mut self.components, delta_t);
        update_players(&mut self.components,self.dimensions.as_ref().unwrap(), delta_t,inputs);
    }

    pub fn draw<T: RaylibDraw>(&self, handle: &mut T)->std::result::Result<(),()>{
        let mut handle2d = handle.begin_mode2D(self.camera.ok_or(())?);
        let dimensions = self.dimensions.as_ref().ok_or(())?;
        draw_tiles(&self.components, dimensions, &mut handle2d);
        draw_player(&self.components, dimensions,&mut handle2d);
        Ok(())
    }

}


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
