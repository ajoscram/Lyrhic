use nannou::{Draw, color::{BLACK, BLUE}};

pub fn draw(draw: &Draw) {
    draw.reset();
    draw.background().color(BLACK);
    
    draw.rect().color(BLUE).w_h(50.0, 50.0).x_y(50.0, 50.0);
}