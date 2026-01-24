use std::iter;
use nannou::{Draw, geom::Rect, image::{self, imageops::FilterType}};
use crate::model::{Args, Drawable, Color, char_reader::CharReader, charxel::Charxel};

pub struct Picture {
    outer_bg: Rect,
    inner_bg: Rect,
    color: Color,
    charxels: Vec<Charxel>,
}

impl Picture {
    pub fn new(args: &Args) -> Self {
        let outer_bg = Rect::from_w_h(args.size() as f32, args.size() as f32);
        let inner_bg = outer_bg.pad(args.margin as f32);
        let container = inner_bg.pad(args.charsize as f32);
        let color = args.bg.clone();
        let charxels = get_charxels(args, &container);
        Self { outer_bg, inner_bg, color, charxels }
    }
}

impl Drawable for Picture {
    fn draw_into(&self, draw: &Draw) {
        let white: Color = Color::from_components((255,255,255,255));

        draw.rect()
            .color(white)
            .xy(self.outer_bg.xy())
            .wh(self.outer_bg.wh());

        draw.rect()
            .color(self.color)
            .xy(self.inner_bg.xy())
            .wh(self.inner_bg.wh());

        for charxel in &self.charxels { charxel.draw_into(draw); }
    }
}

fn get_charxels(args: &Args, container: &Rect) -> Vec<Charxel> {
    let chars = CharReader::new(&args.text).cycle();
    let rects = get_rects(args, container);
    let colors = get_colors(args);
    iter::zip(chars, rects)
        .zip(colors)
        .map(|((char, rect), color)|
            Charxel::new(char, rect, color, args.font.clone()))
        .collect()
}

fn get_colors(args: &Args) -> Vec<Color> {
    image::open(&args.image)
        .unwrap()
        .resize(args.charres, args.charres, FilterType::Gaussian)
        .to_rgba8()
        .pixels()
        .map(|x| Color::from_components((x[0], x[1], x[2], x[3])))
        .collect()
}

fn get_rects(args: &Args, container: &Rect) -> Vec<Rect> {
    let mut current = container;
    let mut rects: Vec<Rect> = Vec::new();

    for y in 0..args.charres {
        for x in 0..args.charres {
            let rect = get_rect(args.charsize as f32, x, y, *container, *current);
            rects.push(rect);
            current = rects.last().unwrap();
        }
    }

    rects
}

fn get_rect(size: f32, x: u32, y: u32, container: Rect, previous: Rect) -> Rect {
    let first_in_row = x == 0;
    let first_in_col = y == 0;
    let mut rect = Rect::from_w_h(size, size);
    
    if first_in_row && first_in_col {
        rect = rect.align_top_of(container).align_left_of(container);
    }
    else if first_in_row && !first_in_col {
        rect = rect.below(previous).align_left_of(container);
    }
    else {
        rect = rect.align_top_of(previous).right_of(previous);
    }

    rect
}