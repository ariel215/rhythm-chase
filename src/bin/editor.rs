use std::ffi::{CStr, CString};
use drawing::RaylibDraw;
use inputs::Location;
use math::Rectangle;
use rhythm_chase::*;
use raylib::*;
use raylib::rgui::RaylibDrawGui;

mod editor{

    use raylib::{color::{self, Color}, drawing::RaylibDraw, ffi::Rectangle, rgui};
    use rhythm_chase::{inputs::Location, rhythm::{self, Rhythm}, tiles::Tile};


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


        pub fn draw<T: RaylibDraw>(&self, handle: &mut T){
            self.draw_grid(handle);
            match self.current_window.as_ref() {
                Some(EditorWindow::Rhythm(editor))=> {editor.draw(handle)},
                Some(EditorWindow::Color(picker)) => {picker.draw(handle)},
                Some(EditorWindow::Text(text_editor)) => {text_editor.draw(handle)},
                None => {()}
            }
            todo!()
        }


        pub fn draw_grid<T: RaylibDraw>(&self, handle: &mut T){
            todo!()
        }
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



fn main() -> Result<(),()>{
    let (mut rl, mut rthred) = raylib::init()
        .height(720)
        .width(480)
        .title("Editor")
        .build();

    let mut text: Vec<u8> = Vec::with_capacity(100_000);
    let bounds = Rectangle{width: 50.0, height: 50.0,x: 50.0, y:50.0};
    while !rl.window_should_close() {
        let mut  handle: prelude::RaylibDrawHandle = rl.begin_drawing(&rthred);
        handle.clear_background(color::Color::WHITE);
        let message = "Lorem ipsum dolor omos";
        let c_message = CString::new(message).or(Err(()))?;
        let font = handle.get_font_default();
        let length = raylib::text::measure_text_ex(font, &message, 10.0, 1.0);
        let bounds = Rectangle {width: length.x, height: length.y, x: 50.0, y:50.0};
        handle.gui_dummy_rec(bounds, Some(&c_message));
    }

    Ok(())
}