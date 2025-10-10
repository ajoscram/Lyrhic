mod save; mod args; mod model;

use nannou::prelude::*;
use model::Model;
use args::{Args, Parser};

fn main() {
    nannou::app(model)
        .update(save) // update has been repurposed to save the image
        .loop_mode(LoopMode::loop_once()) // the image should be saved once only
        .view(view)
        .run();
}

fn model(app: &App) -> Model { model::new(app, Args::parse()) }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window(model.window_id).unwrap().rect();

    frame.clear(BLACK);
    draw.texture(&model.texture).xy(rect.xy()).wh(rect.wh());
    draw.to_frame(app, &frame).unwrap();
}

fn save(app: &App, model: &mut Model, _: Update) {
    let window = app.window(model.window_id).unwrap();
    let output_path = model.args.out.clone();
    save::image(&model.texture, &window, output_path);
}