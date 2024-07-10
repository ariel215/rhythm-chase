use std::ffi::{CStr, CString};
use std::time;
use camera::Camera2D;
use drawing::{RaylibDraw, RaylibMode2DExt};
use inputs::Location;
use math::Rectangle;
use rhythm_chase::*;
use raylib::*;
use raylib::rgui::RaylibDrawGui;

mod editor{

    use raylib::{camera::Camera2D, color::{self, Color}, drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2DExt}, ffi::Rectangle, rgui, RaylibHandle, RaylibThread};
    use rhythm_chase::{inputs::Location, rhythm::{self, Rhythm}, tiles::Tile};


    pub(crate) struct RaylibContext{
        pub handle: RaylibHandle,
        pub thread: RaylibThread
    }

    enum EditorWindow{
        Rhythm(RhythmEditor),
        Color(ColorPicker),
        Text(TextEditor)
    }



    pub struct TileEditor{
        rhythm_editor: Option<RhythmEditor>,
        color_picker: Option<ColorPicker>,
        text_editor: Option<TextEditor>,
        current_window: Option<EditorWindow>
    }

    struct RhythmEditor{}

    impl RhythmEditor {
        fn draw<T: RaylibDraw>(&self, handle: &mut T) {
            todo!()
        }
    }


    impl TileEditor{

        pub fn show_color_picker(&mut self){
            if let Some(picker) = self.color_picker.take(){
                self.current_window = Some(EditorWindow::Color(picker));
            }
            todo!();
        }


        pub fn close_color_picker(&mut self){
            match self.current_window.take() {
                Some(EditorWindow::Color(color_picker)) => {self.color_picker = Some(color_picker);},
                Some(window) => { self.current_window = Some(window)},
                None => {()}
            }
        }

        pub fn set_tile_color<T: RaylibDraw>(&mut self, color: Color){
         
            todo!();
        }

        pub fn show_rhythm_creator(&mut self){
            if let Some(creator) = self.rhythm_editor.take(){
                self.current_window = Some(EditorWindow::Rhythm(creator));
            }
            todo!();
        }

        pub fn set_tile_rhythm<T: RaylibDraw>(&mut self, rhythm: Rhythm) -> Rhythm{
            todo!()
        }

        fn set_text_editor(&mut self){
            if let Some(creator) = self.text_editor.take(){
                self.current_window = Some(EditorWindow::Text(creator));
            }
            todo!();
        }
        
        pub(crate) fn new() -> Self {
            Self { 
                rhythm_editor: None, color_picker: None, text_editor: None, current_window: None }
        }
    }



    pub fn draw_window(ctx: &mut RaylibContext, camera: &Camera2D, tile_editor: &TileEditor){
        let mut handle = ctx.handle.begin_drawing(&ctx.thread);
        handle.clear_background(color::Color::WHITE);

        {
            let mut handle = handle.begin_mode2D(camera);
            // draw grid
            for step in (-2000..2000).step_by(50){
                handle.draw_line(step, 2000, step, -1000, Color::BLACK);
                handle.draw_line(-2000,step, 2000, step, Color::BLACK);
            }
            // todo: draw tiles
        }

        // Draw sidebar: 
        // Draw current tile color 
        

        
        match tile_editor.current_window.as_ref() {
            Some(EditorWindow::Rhythm(editor))=> {editor.draw(&mut handle)},
            Some(EditorWindow::Color(picker)) => {picker.draw(&mut handle)},
            Some(EditorWindow::Text(text_editor)) => {text_editor.draw(&mut handle)},
            None => {()}
        }
        // todo!()
    }


    struct ColorPicker{}


    impl ColorPicker{
        pub fn draw<T: RaylibDraw>(&self, handle: &mut T){
            todo!()
        }
    }

    struct TextEditor{}

    impl TextEditor{
        pub fn draw<T:RaylibDraw>(&self, handle: &mut T){
            todo!()
        }
    }





}

// let message = "Lorem ipsum dolor omos";
// let c_message = CString::new(message).or(Err(()))?;
// let font = handle.get_font_default();
// let length = raylib::text::measure_text_ex(font, &message, 10.0, 1.0);
// let bounds = Rectangle {width: length.x, height: length.y, x: 50.0, y:50.0};
// handle.gui_dummy_rec(bounds, Some(&c_message));
// }




fn main() -> Result<(),()>{
    let window_height = 1280;
    let window_width = 1280;
    let (mut rl, rthred) = raylib::init()
        .height(window_height)
        .width(window_width)
        .title("Editor")
        .build();
    let mut ctx = editor::RaylibContext{
        handle: rl,
        thread: rthred
    };
    let center = math::Vector2 { x: (window_width / 2) as f32, y: (window_height / 2) as f32 };

    let mut text: Vec<u8> = Vec::with_capacity(100_000);
    let bounds = Rectangle{width: 50.0, height: 50.0,x: 50.0, y:50.0};
    let level_editor = editor::TileEditor::new();
    let mut camera = Camera2D{
        offset: center,
        target: center,
        rotation: 0.0,
        zoom: 1.0
    };
    let scroll_speed = 150.0;
    let scroll_border = 50.0;
    while !rl.window_should_close() {
        let delta_t = rl.get_frame_time();
        let mouse_position = rl.get_mouse_position();
        if mouse_position.x < scroll_border {
            camera.target.x -= scroll_speed * delta_t;
        }

        if (window_width as f32) - mouse_position.x < scroll_border {
            camera.target.x += scroll_speed * delta_t
        } 

        if mouse_position.y < scroll_border {
            camera.target.y -= scroll_speed * delta_t;
        }

        if (window_height as f32) - mouse_position.y < scroll_border {
            camera.target.y += scroll_speed * delta_t;
        }

        editor::draw_window(&mut ctx, &camera, &level_editor);
    }
    Ok(())
}