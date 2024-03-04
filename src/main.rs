

use raylib::prelude::*;
use rhythm_chase::*;
use array2d::{Array2D, Error};

fn main() -> Result<(), Error>{
    let (w,h): (i32,i32) = (640,480);
    let (mut rl, thread) = raylib::init()
        .size(w,h)
        .title("Checkerboard")
        .build();
     
    let row1 = (vec![Color::RED, Color::BLACK, Color::RED, Color::BLACK, Color::RED, Color::BLACK, Color::RED, Color::BLACK,])
        .iter().map(Tile::from).collect::<Vec<_>>();
    let row2 = (vec![Color::BLACK, Color::RED, Color::BLACK, Color::RED, Color::BLACK, Color::RED, Color::BLACK, Color::RED])
    .iter().map(Tile::from).collect::<Vec<_>>();
let rows = vec![row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone(), row1.clone(), row2.clone()];
    let rows = Array2D::from_rows(&rows)?;
    let margin = 6;
    let level = Level::new(
        rows,
        w / 8,
        h / 8,
        margin,
        margin
    );
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        level.draw(&mut d);
        
    }
    Ok(())
}