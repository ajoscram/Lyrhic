use clap::Parser;
use nannou::App;

use crate::model::nannou_info::NannouInfo;
use crate::model::args::Args;

pub mod args;
pub mod nannou_info;

pub struct Model {
    pub args: Args,
    pub nannou_info: NannouInfo,
}

impl Model {
    pub fn new(app: &App) -> Model {
        let args = Args::parse();
        let nannou_info = NannouInfo::new(app, &args);
        return Model { args, nannou_info, }
    }
}

