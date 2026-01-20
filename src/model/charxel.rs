use core::fmt;
use std::fmt::Display;
use nannou::{Draw, geom::Rect};
use crate::model::{Drawable, Color};

pub struct Charxel {
    char: char,
    rect: Rect,
    color: Color,
}

impl Charxel {
    pub fn new(char: char, rect: Rect, color: Color) -> Self {
        Self { char, rect, color }
    }
}

impl Display for Charxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | rgba({}, {}, {}, {}) | size({}, {}) | @({}, {})",
            self.char,
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha,
            self.rect.w(),
            self.rect.h(),
            self.rect.x(),
            self.rect.y())
    }
}

impl Drawable for Charxel{
    fn draw_into(&self, draw: &Draw) {
        draw.rect()
            .color(self.color)
            .xy(self.rect.xy())
            .wh(self.rect.wh());
    }
}