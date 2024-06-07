
use raylib::prelude::*;
use std::{io, thread::{sleep}, time::*};
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


const WINDOW_HEIGHT: i32 = 640;
const WINDOW_WIDTH: i32 = 480;

fn main() -> Result<(), Error>{
    let (w,h): (i32,i32) = (640,480);
    let (mut rl, thread) = raylib::init()
        .size(w,h)
        .title("Checkerboard")
        .resizable()
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
    // let json = std::fs::File::open("maps/bigmap.json")?;
    // let mut level: Level = serde_json::from_reader(io::BufReader::new( json))?;
    // level.player = Player::new((0,0), 120., level.size_tiles(), 0.15);
    // let mut time = SystemTime::now();
    while !rl.window_should_close() {
        // let duration = SystemTime::now().duration_since(time).unwrap();
        // time = SystemTime::now();
        // let inputs = rhythm_chase::inputs::get_inputs(&mut rl);
        // level.update(duration.as_secs_f64(), &inputs);
        {
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            let width = d.get_screen_width();
            let height = d.get_screen_height();
            d.clear_background(Color::WHITE);
            // draw_txt_centered(&mut d, "hello world\nhello again", height / 10);
            draw_txt_centered(&mut d, 
                "hello world I hope you have a good day here is some more text \nand then some more let's just see how long we can make this thing go!!!",
            height / 10);
            // draw_txt(&mut d, "hello world\nI hope you have a good day\nhere is a third line as well\nwhat if we just keep going\nand never ever stop");
        }   
    }
    Ok(())
}


fn draw_txt_centered(handle: &mut RaylibDrawHandle, text: &str, y: i32){
    let width = handle.get_screen_width();
    let font_size = 18;
    let margin = 15;
    let text_space = 3 * width / 5 - (2 * margin);
    let space_size = measure_text(" ", font_size);
    let mut contents = vec![vec![]];
    for line in text.lines(){
        let mut length = 0;
        for word in line.split_ascii_whitespace(){
            let word_length = measure_text(word, font_size) + space_size;
            if length +  word_length < text_space{
                length += word_length;
                let clen = contents.len() - 1;
                contents[clen].push(word)
            } else {
                length = word_length;
                contents.push(vec![word])
            }
        }
        contents.push(vec![]);
    }
    let contents: Vec<_> = contents.iter().map(|v|v.join(" ")).collect();
    let n_lines = contents.len() as i32;
    let height = (4 * (font_size + 2) / 3) * (n_lines+1); // would love to find a more principled way to get this to be the right size
    let start_x = width / 5 + margin;
    let start_y = y + font_size;
    let text = contents.join("\n");
    handle.draw_rectangle(
        width/ 5,
        y,
        3 * width / 5,
        height, Color::GRAY);
    handle.draw_text(&text, start_x,start_y , font_size, Color::BLACK)
}