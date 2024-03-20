
use raylib::prelude::*;
use std::{io, time::*};
use rhythm_chase::*;

#[derive(Debug)]
enum Error{
    Array2D(array2d::Error),
    IO(std::io::Error),
    Json(serde_json::Error)
}

impl  From<array2d::Error> for Error {
    fn from(value: array2d::Error) -> Self {
        Error::Array2D(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}


fn main() -> Result<(), Error>{
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
    let json = std::fs::File::open("maps/begin.json")?;
    let mut level: Level = serde_json::from_reader(io::BufReader::new( json))?;
    level.player = Player::new((0,0), 120., level.size(), 0.15);
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