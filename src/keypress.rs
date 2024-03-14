use mki;
use std::{collections::*, str::FromStr};

use crate::Sec;

struct KeyState {
    is_down: bool,
    just_pressed: bool,
}

pub struct KeyPressDetector {
    keys: HashMap<mki::Keyboard, KeyState>,
    period: Sec
}

impl KeyPressDetector {
    pub fn new<'a>(to_check: impl Iterator<Item=&'a str>, period: Sec) -> Result<Self,()>{
        let mut keys = to_check.map(mki::Keyboard::from_str);
        if keys.any(|r|r.is_err()){
            return Err(())
        }
        // We know its OK to unwrap all the keys
        Ok(KeyPressDetector{
            keys: keys.map(|r|
                (r.unwrap(),KeyState{is_down: false, just_pressed: false})
            ).collect(),
            period
        })
    }

    pub fn update(&mut self) {
        for (key, prev_state) in self.keys.iter_mut() {
            let is_pressed = mki::are_pressed(&[*key]);
            if is_pressed{
                ();
            }
            prev_state.just_pressed = is_pressed & !prev_state.is_down;
            prev_state.is_down = is_pressed; 
        }
    }

}