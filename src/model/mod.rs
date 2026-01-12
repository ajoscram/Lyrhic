mod args; mod picture; mod canvas;

use clap::Parser;
use nannou::{App, Draw};
use crate::model::{canvas::Canvas, picture::Picture, args::Args};

pub struct Model {
    pub args: Args,
    pub canvas: canvas::Canvas,
    pub picture: picture::Picture,
}

impl Model {
    pub fn new(app: &App) -> Model {
        let args = Args::parse();
        let canvas = Canvas::new(app, &args);
        let picture = Picture::new(&args);
        return Model { args, canvas, picture }
    }
}

pub(crate) trait Drawable {
    fn draw_into(&self, draw: &Draw);
}