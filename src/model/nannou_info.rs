use std::cell::Ref;
use nannou::{draw::{Renderer, RendererBuilder}, prelude::*, wgpu::{Texture, TextureBuilder, TextureFormat, TextureReshaper, TextureUsages}};
use crate::model::args::Args;

const PREVIEW_SIZE: u32 = 512;

pub struct NannouInfo {
    pub draw: Draw,
    pub renderer: Renderer,
    pub texture: Texture,
    pub texture_reshaper: TextureReshaper,
    window_id: WindowId,
}

impl NannouInfo {
    pub fn new(app: &App, args: &Args) -> NannouInfo {
        let window_id = get_window_id(app);
        let window = get_window(app, window_id);
        let texture = get_texture(args, &window);
        let texture_reshaper = get_texture_reshaper(&texture, &window);
        let renderer = RendererBuilder::new().build_from_texture_descriptor(
            window.device(),
            texture.descriptor());
        
        return NannouInfo {
            window_id,
            draw: Draw::new(),
            renderer,
            texture,
            texture_reshaper,
        }
    }

    pub fn window<'a>(&self, app: &'a App) -> Ref<'a, Window> { get_window(app, self.window_id) }
}

fn get_window_id(app: &App) -> WindowId {
    return app
        .new_window()
        .size(PREVIEW_SIZE, PREVIEW_SIZE)
        .build()
        .unwrap();
}

fn get_window<'a>(app: &'a App, window_id: WindowId) -> Ref<'a, Window> { app.window(window_id).unwrap() }

fn get_texture(args: &Args, window: &Window) -> Texture {
    return TextureBuilder::new()
        .size([args.size, args.size])
        .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        .sample_count(window.msaa_samples())
        .format(TextureFormat::Rgba16Float)
        .build(window.device());
}

fn get_texture_reshaper(texture: &Texture, window: &Window) -> TextureReshaper {
    return TextureReshaper::new(
        window.device(),
        &texture.view().build(),
        window.msaa_samples(),
        texture.sample_type(),
        window.msaa_samples(),
        Frame::TEXTURE_FORMAT,
    );
}