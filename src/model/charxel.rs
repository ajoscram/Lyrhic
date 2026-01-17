use core::fmt;
use std::fmt::Display;
use nannou::{Draw, image::{Primitive, Rgba}};
use crate::model::Drawable;

pub struct Charxel<T> where T: Primitive + Display {
    char: char,
    color: Rgba<T>,
}

impl<T: Primitive + Display> Charxel<T> {
    pub fn new(char: char, color: Rgba<T>) -> Self {
        Self { char, color }
    }
}

impl<T: Primitive + Display> Display for Charxel<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "({}, {}, {}, {}) -> {}",
            self.color[0],
            self.color[1],
            self.color[2],
            self.color[3],
            self.char)
    }
}

impl<T: Primitive + Display> Drawable for Charxel<T>{
    fn draw_into(&self, draw: &Draw) {
    }
}