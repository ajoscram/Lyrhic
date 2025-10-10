use nannou::{image::{self, imageops::FilterType}, prelude::*, wgpu::Texture};
use crate::Args;

pub struct Model {
    pub args: Args,
    pub window_id: WindowId,
    pub texture: Texture,
}

pub fn new(app: &App, args: Args) -> Model {
    let window_id = app
        .new_window()
        .size(1080, 1080)
        .build()
        .unwrap();

    let image = image::open(&args.image)
        .unwrap()
        .grayscale()
        .resize(args.size, args.size, FilterType::Gaussian);

    // this texture should contain the letters and shit
    let texture = Texture::from_image(app, &image);

    return Model {
        args,
        window_id,
        texture
    }
}
