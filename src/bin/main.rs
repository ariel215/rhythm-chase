
use raylib::prelude::*;
use std::{io, time::*};
use rhythm_chase::*;



fn main() -> Result<(), rhythm_chase::RCError>{
    let (w,h): (i32,i32) = (640,480);
    let (mut rl, thread) = raylib::init()
        .size(w,h)
        .title("Checkerboard")
        .build();
    // let downbeat = TileRhythm::new(2, 120., [0]);
    // let upbeat = TileRhythm::new(2,120.,[1]);
    // let red = Tile::from(&Color::RED, Some(downbeat));
    // let black: Tile = Tile::from(&Color::BLACK, Some(upbeat));
    
    // let row1 = vec![red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone()];
    // let row2 = vec![black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone()];
    // let rows = vec![row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone()];
    // let rows = array2d::Array2D::from_rows(&rows)?;
    // let margin = 6;
    // let level = Level::new(
    //     rows,
    //     w / 8,
    //     h / 8,
    //     margin,
    //     margin
    // );
    // let player = level.player.clone();
    // let json  = std::fs::File::create("level.json")?;
    // serde_json::to_writer_pretty(json, &level)?;
    
    let dimensions = TileDimensions{tile_width: 80,tile_height: 80,row_gap: 3, column_gap: 3};
    let camera = Camera2D{
        offset: Vector2 { x: (w/2) as f32, y: (h / 2) as f32 },
        target: Vector2 {x: (w/2) as f32, y: (h /2) as f32},
        rotation: 0.0,
        zoom: 1.0
    };
    let mut game = Game::new(camera, dimensions);
    game.load_level("maps/begin.json")?;
    let mut time = SystemTime::now();
    while !rl.window_should_close() {
        let duration = SystemTime::now().duration_since(time).unwrap();
        time = SystemTime::now();
        let inputs = rhythm_chase::inputs::get_inputs(&mut rl);
        game.update(duration.as_secs_f64(), &inputs);
        {
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            game.draw(&mut d);
        }
    }
    Ok(())
}