

use raylib::prelude::*;
use std::time::{self, *};
use rhythm_chase::*;
use array2d::{Array2D, Error};

fn main() -> Result<(), Error>{
    let (w,h): (i32,i32) = (640,480);
    let (mut rl, thread) = raylib::init()
        .size(w,h)
        .title("Checkerboard")
        .build();
    let downbeat = TileRhythm::new(2, 120., [0]);
    let upbeat = TileRhythm::new(2,120.,[1]);
    let red = Tile::from(&Color::RED, Some(downbeat));
    let black: Tile = Tile::from(&Color::BLACK, Some(upbeat));
    
    let row1 = vec![red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone()];
    let row2 = vec![black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone(), black.clone(), red.clone()];
    let rows = vec![row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone()];
    let rows = Array2D::from_rows(&rows)?;
    let margin = 6;
    let mut level = Level::new(
        rows,
        w / 8,
        h / 8,
        margin,
        margin
    );
    let mut time = SystemTime::now();
    while !rl.window_should_close() {
        let duration = SystemTime::now().duration_since(time).unwrap();
        time = SystemTime::now();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        level.update(duration.as_secs_f64());
        level.draw(&mut d);
    }
    Ok(())
}