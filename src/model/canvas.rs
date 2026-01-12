use std::{cell::Ref, path::PathBuf};
use nannou::{draw::{Renderer, RendererBuilder}, prelude::*, wgpu::{CommandEncoder, CommandEncoderDescriptor, Texture, TextureBuilder, TextureCapturer, TextureFormat, TextureReshaper, TextureSnapshot, TextureUsages}};
use crate::model::{args::Args, Drawable};

const PREVIEW_SIZE: u32 = 512;

pub struct Canvas {
    draw: Draw,
    window_id: WindowId,
    renderer: Renderer,
    texture: Texture,
    texture_reshaper: TextureReshaper,
}

impl Canvas {
    pub fn new(app: &App, args: &Args) -> Canvas {
        let window_id = get_window_id(app);
        let window = get_window(app, window_id);
        let texture = get_texture(args, &window);
        let texture_reshaper = get_texture_reshaper(&texture, &window);
        let renderer = RendererBuilder::new().build_from_texture_descriptor(
            window.device(),
            texture.descriptor());
        
        return Canvas {
            window_id,
            draw: Draw::new(),
            renderer,
            texture,
            texture_reshaper,
        }
    }

    pub fn draw(&self, drawable: &impl Drawable) { drawable.draw_into(&self.draw); }

    pub fn show(&self, frame: Frame) {
        self.texture_reshaper.encode_render_pass(
            frame.texture_view(),
            &mut *frame.command_encoder());
    }

    pub fn save(&mut self, app: &App, output_path: PathBuf) {
        let window = get_window(app, self.window_id);
        let device = window.device();
        let descriptor = CommandEncoderDescriptor { label: Some("Lyrhic image renderer"), };
        let mut encoder = device.create_command_encoder(&descriptor);

        self.renderer.render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        take_snapshot(&self.texture, &window, encoder)
            .read(|result| {
                result
                    .expect("Failed to map texture memory.")
                    .to_owned()
                    .save(output_path)
                    .expect("Failed to save texture to PNG image.");
            })
            .unwrap();
    }
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

fn take_snapshot(texture: &Texture, window: &Window, mut encoder: CommandEncoder) -> TextureSnapshot {
    let device = window.device();
    let texture_capturer = TextureCapturer::default();
    let snapshot = texture_capturer.capture(device, &mut encoder, &texture);

    window.queue().submit(Some(encoder.finish()));
    texture_capturer.await_active_snapshots(&device).unwrap();

    return snapshot;
}