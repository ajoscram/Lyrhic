use nannou::Draw;
use nannou::color::{BLACK, BLUE};

use crate::model::Drawable;
use crate::model::Args;

pub struct Picture {

}

impl Picture {
    pub fn new(args: &Args) -> Picture {
        Picture {
            
        }
    }
}

impl Drawable for Picture {
    fn draw_into(&self, draw: &Draw) {
        draw.reset();
        draw.background().color(BLACK);
        
        draw.rect().color(BLUE).w_h(50.0, 50.0).x_y(50.0, 50.0);
    }
}