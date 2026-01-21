mod model;

use nannou::prelude::*;
use model::Model;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model { Model::new(app) }

fn view(app: &App, model: &Model, frame: Frame) { model.canvas.show(app, frame); }

fn update(app: &App, model: &mut Model, _: Update) {
    
    println!("Drawing...");
    model.canvas.draw(&model.picture);

    println!("Rendering...");
    let snapshot = model.canvas.render(app);

    println!("Saving...");
    model.canvas.save(app, snapshot, &model.args.out);

    println!("Done!");
}