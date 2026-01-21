use std::{cell::Ref, path::PathBuf};
use nannou::{draw::{Renderer, RendererBuilder}, prelude::*, wgpu::{CommandEncoderDescriptor, Texture, TextureBuilder, TextureCapturer, TextureFormat, TextureReshaper, TextureSnapshot, TextureUsages}};
use crate::model::{Args, Drawable};

const PREVIEW_SIZE: u32 = 512;

pub struct Canvas {
    draw: Draw,
    window_id: WindowId,
    renderer: Renderer,
    texture: Texture,
    capturer: TextureCapturer,
    reshaper: TextureReshaper,
}

impl Canvas {
    pub fn new(app: &App, args: &Args) -> Self {
        let window_id = get_window_id(app);
        let window = get_window(app, window_id);
        let texture = get_texture(args, &window);
        let reshaper = get_texture_reshaper(&texture, &window);
        let renderer = RendererBuilder::new().build_from_texture_descriptor(
            window.device(),
            texture.descriptor());
        
        return Self {
            window_id,
            draw: Draw::new(),
            renderer,
            texture,
            capturer: TextureCapturer::default(),
            reshaper,
        }
    }

    pub fn draw(&self, drawable: &impl Drawable) { drawable.draw_into(&self.draw); }

    pub fn render(&mut self, app: &App) -> TextureSnapshot {
        let window = get_window(app, self.window_id);
        let device = window.device();
        let descriptor = CommandEncoderDescriptor { label: Some("Lyrhic image renderer"), };
        let mut encoder = device.create_command_encoder(&descriptor);

        self.renderer.render_to_texture(device, &mut encoder, &self.draw, &self.texture);
        let snapshot = self.capturer.capture(device, &mut encoder, &self.texture);
        window.queue().submit(Some(encoder.finish()));

        snapshot
    }

    pub fn show(&self, frame: Frame) {
        self.reshaper.encode_render_pass(
            frame.texture_view(),
            &mut *frame.command_encoder());
    }

    pub fn save(&self, app: &App, snapshot: TextureSnapshot, output_path: &PathBuf) {
        let window = get_window(app, self.window_id);
        let device = window.device();
        let output_path_clone = output_path.clone();

        snapshot
            .read(|result| {
                result
                    .expect("Failed to map texture memory.")
                    .to_owned()
                    .save(output_path_clone)
                    .expect("Failed to save texture to PNG image.");
            })
            .unwrap();

        // TODO: await_active_snapshots is not working as intended.
        // This seems to do something for now, but needs to get removed eventually
        println!("THIS SHOULDN'T BE HERE, NOR DISPLAY 0: {}", self.capturer.active_snapshots());
        self.capturer.await_active_snapshots(device).unwrap();
        println!("THIS SHOULDN'T BE HERE EITHER.");
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