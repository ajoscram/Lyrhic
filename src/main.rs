mod save; mod model; mod draw;

use nannou::prelude::*;
use model::Model;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .view(view)
        .update(execute)
        .run();
}

fn model(app: &App) -> Model { Model::new(app) }

fn execute(app: &App, model: &mut Model, _: Update) {
    draw::draw(&model.nannou_info.draw);
    save::to_image(
        app,
        &mut model.nannou_info,
        model.args.out.clone());
}

fn view(_: &App, model: &Model, frame: Frame) {
    model.nannou_info.texture_reshaper.encode_render_pass(
        frame.texture_view(),
        &mut *frame.command_encoder());
}