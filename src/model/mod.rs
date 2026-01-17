pub mod args; mod picture; mod canvas; mod charxel; mod char_reader;

use clap::Parser;
use nannou::{App, Draw};
use crate::model::{canvas::Canvas, picture::Picture, args::Args};

pub struct Model {
    pub args: Args,
    pub canvas: Canvas,
    pub picture: Picture,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let args = Args::parse();
        let canvas = Canvas::new(app, &args);
        let picture = Picture::new(&args);
        Self { args, canvas, picture }
    }
}

pub(crate) trait Drawable {
    fn draw_into(&self, draw: &Draw);
}