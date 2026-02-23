use core::fmt;
use std::fmt::Display;
use nannou::{Draw, geom::Rect, text::*};
use crate::model::{Drawable, Color};

pub struct Charxel {
    char: char,
    rect: Rect,
    color: Color,
    font: Font,
}

impl Charxel {
    pub fn new(char: char, rect: Rect, color: Color, font: Font) -> Self {
        Self { char, rect, color, font }
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

impl Drawable for Charxel {
    fn draw_into(&self, draw: &Draw) {
        draw
            .text(&self.char.to_string())
            .font(self.font.clone())
            .font_size(self.rect.w() as u32)
            .color(self.color)
            .center_justify()
            .align_text_middle_y()
            .xy(self.rect.xy());
    }
}