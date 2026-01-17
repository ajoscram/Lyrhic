use std::iter;
use nannou::{Draw, color, image::{self, Rgba, imageops::FilterType}};
use crate::model::{Args, Drawable, char_reader::CharReader, charxel::Charxel};

pub struct Picture {
    charxels: Vec<Charxel<u8>>
}

impl Picture {
    pub fn new(args: &Args) -> Self {
        let charxels = get_charxels(args);
        for charxel in &charxels { println!("{}", charxel); }
        Self { charxels }
    }
}

impl Drawable for Picture {
    fn draw_into(&self, draw: &Draw) {
        draw.background().color(color::ANTIQUEWHITE);
        draw.rect().color(color::BLUE).w_h(50.0, 50.0).x_y(0.0, 0.0);
    }
}

fn get_charxels(args: &Args) -> Vec<Charxel<u8>> {
    let chars = CharReader::new(&args.text).cycle();
    let pixels = get_pixels(args);
    iter::zip(chars, pixels)
        .map(|(char, pixel)| Charxel::new(char, pixel))
        .collect()
}

fn get_pixels(args: &Args) -> Vec<Rgba<u8>> {
    image::open(&args.image)
        .unwrap()
        .grayscale()
        .resize(args.size / 4, args.size / 4, FilterType::Gaussian)
        .to_rgba8()
        .pixels()
        .map(|x| *x)
        .collect()
}