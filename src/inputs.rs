use raylib;

pub type Location = (i32,i32);

pub enum Input{
    Key(raylib::consts::KeyboardKey),
    Selection(Location)
}


pub fn get_inputs(rh: &mut raylib::RaylibHandle) -> Vec<Input> {
    let mut inputs = vec!();
    while let Some(key) = rh.get_key_pressed(){
        inputs.push(Input::Key(key));
    }
    if rh.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON){
        inputs.push(Input::Selection((rh.get_mouse_x(),rh.get_mouse_y())));
    }

    inputs
}